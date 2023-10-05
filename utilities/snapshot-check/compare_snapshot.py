#!/usr/bin/env python3
"""
Simple program to compare snapshots.
"""

from __future__ import annotations

import argparse
import sys
import json
import yaml

from pathlib import Path
from typing import Any
from pprint import pprint

def is_dir(dirpath: str | Path):
    """Check if the directory is a directory."""
    real_dir = Path(dirpath)
    if real_dir.exists() and real_dir.is_dir():
        return real_dir
    raise argparse.ArgumentTypeError(f"{dir} is not a directory.")


def is_file(filename: str):
    """Does the path exist and is it a file"""
    real_filename = Path(filename).relative_to(".")
    is_dir(real_filename.parent)
    if real_filename.is_dir():
        raise argparse.ArgumentTypeError(f"{filename} is not a file.")
    return real_filename

def normalize_snapshot(snapshot) -> dict:
    normalized={}

    if isinstance(snapshot, list):
        for rec in snapshot:
            normalized[rec["hir"]["address"]] = rec["hir"]["voting_power"]
    else:
        # legacy snapshot
        for rec in snapshot["initial"][0]["fund"]:
            normalized[rec["address"]] = rec["value"]

    return normalized


def analyze_snapshot(args: argparse.Namespace):
    """Compare snapshots.  ONLY checks if the address and voting power match."""
    snapshot = normalize_snapshot(json.loads(args.snapshot.read_text()))
    compare = normalize_snapshot(json.loads(args.compare.read_text()))

    print(f"Found {len(snapshot.keys())} voters in Snapshot")
    print(f"Found {len(compare.keys())} voters in Compare")


    # Read Genesis data
    genesis = None
    if args.genesis:
        print("Reading Genesis Data")
        genesis = {}
        raw_genesis = yaml.unsafe_load(args.genesis.read_text())
        
        for rec in raw_genesis["initial"]:
            if "token" in rec:
                addr = rec["token"]["to"][0]["address"]
                value = rec["token"]["to"][0]["value"]
                if addr not in genesis:
                    genesis[addr] = value
                else:
                    print(f"Address already exists - {addr} - Double voting Key")
    
    if genesis is None:
        print("Genesis records not available")
    else:
        print(f"Found {len(genesis.keys())} voters in Genesis")
                
    for key in snapshot:
        snap = snapshot[key]
        if key not in compare:
            print(f"Voter Address not found {key}")
        else:
            ok = False
            cmp = compare.pop(key)
            # print(f"Voter {key} {snapshot[key]} == {cmp}");
            if cmp == snap:
                ok = True
            elif (cmp > snap) and int(cmp / 1000000) == snap:
                ok = True
            elif (cmp < snap) and int(snap/ 1000000) == cmp:
                ok = True

            if not ok:
                if cmp > snap:
                    print(int(cmp / 1000000))
                if snap > cmp:
                    print(int(snap / 1000000))
                print(f"Voter {key} {snapshot[key]} != {cmp}")


        if genesis is not None:
            if key not in genesis:
                print(f"Snapshot Key not found in Genesis: {key}")
            else:
                genesis_value = genesis.pop(key)
                ok = False
                if genesis_value == snap:
                    ok = True
                if (genesis_value > snap) and int(genesis_value / 1000000) == snap:
                    ok = True
                if (genesis_value < snap) and int(snap / 1000000) == genesis_value:
                    ok = True
                
                if not ok:
                    print(f"Genesis {key} {genesis_value} != {snap}")
        else:
            print("Genesis records not available")
            return

    for key in compare:
        print(f"Comparison Voter Address not found {key}")

    for key, value in genesis.items():
        print(f"Genesis Voter Address not found {key}: {value}")        
    



def main() -> int:
    """Parse CLI arguments."""
    parser = argparse.ArgumentParser(
        description="Compare fully processed snapshots."
    )
    parser.add_argument(
        "--snapshot",
        help="Snapshot file to read.",
        required=True,
        type=is_file,
    )

    parser.add_argument(
        "--compare",
        help="Snapshot file to compare with.",
        required=False,
        type=is_file,
    )

    parser.add_argument(
        "--genesis",
        help="Genesis YAML file to compare with.",
        required=False,
        type=is_file,
    )

    args = parser.parse_args()
    analyze_snapshot(args)
    return 0


if __name__ == "__main__":
    sys.exit(main())