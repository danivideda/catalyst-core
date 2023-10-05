#!/usr/bin/env python3
"""
Simple program to compare snapshots.
"""

from __future__ import annotations

import argparse
import sys
import json

from pathlib import Path
from typing import Any

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
    raw_snapshot = json.loads(args.raw_snapshot.read_text())
    vitss_snapshot = json.loads(args.vitss_snapshot.read_text())
    processed_snapshot = json.loads(args.processed_snapshot.read_text())
    voters_hex = json.loads(args.voters_hex.read_text())
    voters_ca = json.loads(args.voters_ca.read_text())
    
    # Processed is the authority snapshot
    all_ca_voters = {}
    all_hex_voters = {}
    for rec in processed_snapshot:
        if rec["hir"]["address"] not in all_ca_voters:
            all_ca_voters[rec["hir"]["address"]] = rec
        else:
            print(f"Address {rec['hir']['address']} already exists - Double voting Key")

        # Just an index to get the address from the hex key
        if rec["hir"]["voting_key"] not in all_hex_voters:
            all_hex_voters[rec["hir"]["voting_key"]] = rec["hir"]["address"]
        else:
            print(f"Address {rec['hir']['voting_key']} already exists - Double voting Key")            
            
    # Add the vitss snapshot data to the processed, should end up 1:1 - checked later
    for rec in vitss_snapshot["initial"][0]["fund"]:
        if rec["address"] not in all_ca_voters:
            print(f"Address {rec['address']} is not present in processed snapshot - Extra voting Key")
        else:
            all_ca_voters[rec["address"]]["vitss"] = rec
            
    missing_from_vitss = 0
    for _, rec in all_ca_voters.items():
        if rec["vitss"] is None:
            missing_from_vitss += 1
            print(f"Address {rec['hir']['address']} is not present in vitss snapshot - Missing voting Key")
            
    # mark each possible voter if they voted with the hex key.
    for addr in voters_hex:
        if addr not in all_hex_voters:
            print(f"Voted Key {addr} is not present in processed snapshot - Extra voting Key")
        else:
            all_ca_voters[all_hex_voters[addr]]["hex_voted"] = True
            
    for addr in voters_ca:
        if addr not in all_ca_voters:
            print(f"Voted Address {addr} is not present in processed snapshot - Extra voting Key")
        else:
            all_ca_voters[addr]["addr_voted"] = True
        
    voted = 0
    not_voted = 0
    inconsistent_voted = 0    
    for _, rec in all_ca_voters.items():
        if "hex_voted" in rec:
            if "addr_voted" in rec:
                voted += 1
        elif "addr_voted" in rec:
            inconsistent_voted += 1
            print(f"{rec['hir']['address']}/{rec['hir']['voting_key']} voted inconsistently")
        else:
            not_voted += 1    
            
    raw_vp = {}
    for rec in raw_snapshot:
        if isinstance(rec["delegations"], str):
            addr = rec["delegations"]
        else:
            if len(rec["delegations"]) == 1:
                addr = rec["delegations"][0][0]
            else:
                print(f"Address {rec['delegations']} is not formatted properly")
                addr = None

        if addr not in raw_vp:
            raw_vp[addr[2:]] = rec["voting_power"]
        else:
            raw_vp[addr[2:]] += rec["voting_power"]

    # Check processed snapshot has all the voters it should.
    raw_not_found = 0
    raw_voted = 0
    raw_not_voted = 0
    underthreshold = 0
    total_raw = len(raw_vp)
    for key, vp in raw_vp.items():
        if key not in all_hex_voters:
            if vp > 450000000:
                raw_not_found += 1
            else:
                underthreshold += 1
        else:
            addr = all_hex_voters[key]
            if "hex_voted" not in all_ca_voters[addr]:
                raw_not_voted += 1
            else:
                raw_voted += 1
            
    total_extra_keys = 0
    for key in all_hex_voters:
        if key not in raw_vp:
            print(f"Voted Key {key} is not present in raw snapshot - Extra voting Key")
            total_extra_keys += 1
            
    print(f"Voted = {voted}")
    print(f"Not Voted = {not_voted}")
    print(f"Inconsistent Voted = {inconsistent_voted}")
    print(f"Missing From Vitss = {missing_from_vitss}")
    
    print(f"Raw Not Found = {raw_not_found}")
    print(f"Raw Voted = {raw_voted}")
    print(f"Raw Not Voted = {raw_not_voted}")        
    print(f"Under Threshold = {underthreshold}")    
    print(f"Total Raw = {total_raw}")
    print(f"Total Extra Keys = {total_extra_keys}")

def main() -> int:
    """Parse CLI arguments."""
    parser = argparse.ArgumentParser(
        description="Compare fully processed snapshots."
    )
    parser.add_argument(
        "--raw_snapshot",
        help="Snapshot file to read.",
        required=True,
        type=is_file,
    )

    parser.add_argument(
        "--vitss_snapshot",
        help="Vitss Snapshot file to compare with.",
        required=True,
        type=is_file,
    )
    
    parser.add_argument(
        "--processed_snapshot",
        help="Processed Snapshot file to compare with.",
        required=True,
        type=is_file,
    )
    
    parser.add_argument(
        "--voters_hex",
        help="List of hex keys that voted.",
        required=True,
        type=is_file,
    )

    parser.add_argument(
        "--voters_ca",
        help="List of ca keys that voted.",
        required=True,
        type=is_file,
    )

    args = parser.parse_args()
    analyze_snapshot(args)
    return 0


if __name__ == "__main__":
    sys.exit(main())