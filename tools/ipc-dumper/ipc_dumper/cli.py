from __future__ import annotations

import argparse
import os
import shutil
import signal
import subprocess
from datetime import datetime
from pathlib import Path

from .beautify import write_beautified_dump


DEFAULT_SOCKET_PATH = Path("/run/cloudflare-warp/warp_service")
DEFAULT_BACKUP_SUFFIX = ".bak"
DEFAULT_OUTPUT_PATH = Path(".dumps/raw.dump")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Mirror the Warp IPC socket through socat and append the hexdump to a file.",
    )
    parser.add_argument(
        "--socket-path",
        type=Path,
        default=DEFAULT_SOCKET_PATH,
        help="Path to the live Warp socket.",
    )
    parser.add_argument(
        "--backup-suffix",
        default=DEFAULT_BACKUP_SUFFIX,
        help="Suffix to use for the moved original socket.",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_OUTPUT_PATH,
        help="File to append the socat dump to.",
    )
    parser.add_argument(
        "--socat-bin",
        default="socat",
        help="Path to the socat executable.",
    )
    parser.add_argument(
        "--beautified-output",
        type=Path,
        default=None,
        help="Optional path to write a beautified version of the raw dump after socat exits.",
    )
    return parser


def ensure_root() -> None:
    if hasattr(os, "geteuid") and os.geteuid() != 0:
        raise SystemExit("This script must be run as root because it renames the Warp socket.")


def rotate_existing_backup(backup_path: Path) -> None:
    if not backup_path.exists():
        return

    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    rotated_path = backup_path.with_name(f"{backup_path.name}.{stamp}")
    backup_path.rename(rotated_path)


def move_socket(socket_path: Path, backup_suffix: str) -> Path:
    if not socket_path.exists():
        raise SystemExit(f"Socket path does not exist: {socket_path}")

    backup_path = socket_path.with_name(f"{socket_path.name}{backup_suffix}")
    rotate_existing_backup(backup_path)
    socket_path.rename(backup_path)
    return backup_path


def restore_socket(socket_path: Path, backup_path: Path) -> None:
    if not backup_path.exists():
        return

    if socket_path.exists():
        socket_path.unlink()

    backup_path.rename(socket_path)


def start_socat(socket_path: Path, backup_path: Path, output_path: Path, socat_bin: str) -> int:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    command = [
        socat_bin,
        "-x",
        "-v",
        f"UNIX-LISTEN:{socket_path},fork,mode=777",
        f"UNIX-CONNECT:{backup_path}",
    ]

    with output_path.open("a", encoding="utf-8") as log_file:
        log_file.write(
            f"\n=== warp-ipc-dumper started {datetime.now().isoformat(timespec='seconds')} ===\n"
        )
        log_file.flush()

        process = subprocess.Popen(
            command,
            stdout=log_file,
            stderr=subprocess.STDOUT,
            text=True,
        )

        try:
            return process.wait()
        except KeyboardInterrupt:
            process.send_signal(signal.SIGINT)
            return process.wait()
        finally:
            restore_socket(socket_path, backup_path)


def maybe_write_beautified_output(raw_output_path: Path, beautified_output_path: Path | None) -> None:
    if beautified_output_path is None:
        return

    write_beautified_dump(raw_output_path, beautified_output_path)


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    ensure_root()

    socat_path = shutil.which(args.socat_bin)
    if socat_path is None:
        raise SystemExit(f"Could not find socat executable: {args.socat_bin}")

    backup_path = move_socket(args.socket_path, args.backup_suffix)
    exit_code = start_socat(args.socket_path, backup_path, args.output, socat_path)
    maybe_write_beautified_output(args.output, args.beautified_output)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())