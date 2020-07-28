#!/usr/bin/env python3

# This script is responsible for running build.py
# and uploading the output files to github's "releases"
# section

import os
import sys
import subprocess

# Stolen from https://stackoverflow.com/a/60878313, thanks for such a succinct and pythonic impl!
def read_in_secrets():
  with open('.secret_env', 'r') as fh:
    vars_dict = dict(
        tuple([x.strip() for x in line.split('=')])
        for line in fh.readlines() if not line.startswith('#')
    )
    os.environ.update(vars_dict)

def read_in_version():
  with open('Cargo.toml', 'r') as fh:
    for line in fh.readlines():
      if line.startswith("version"):
        version_str = line.strip().split("=")[1].strip()
        version_str = version_str.replace('"', '')
        return version_str
  return '0.0.0'

def main(argv):
  subprocess.run([sys.executable, 'build.py']).check_returncode()
  
  read_in_secrets()
  version = read_in_version()

  # Has this version been created on GH yet?
  
  print("Uploading {}".format(version))

  release_files = [
    "target/x86_64-unknown-linux-gnu/release/meili",
    "target/x86_64-pc-windows-gnu/release/meili.exe",
    "target/x86_64-apple-darwin/release/meili",
  ]
  for r in release_files:
    print("TODO see https://developer.github.com/changes/2013-09-25-releases-api/ ")


if __name__ == '__main__':
  main(sys.argv)


