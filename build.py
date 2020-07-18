#!/usr/bin/env python3

# This script is responsible for checking dependencies, offering to install dependencies,
# running builds for all target platforms (linux/windows/macos),
# and finally pushing the builds to someplace (in my case the ODU cs server)

import os
import sys
import subprocess
import platform

def check_dependencies():
  print("TODO implement something useful in check_dependencies");

def build_for(target_triple):
  print("Building ./target/{}/release/meili".format(target_triple))
  subprocess.run([
    'cargo', 'build', '--release', '--target={}'.format(target_triple)
  ]).check_returncode()
  # ^^ Throws CalledProcessError if cargo failed to build something.
  

def main(argv):
  check_dependencies()

  build_for("x86_64-unknown-linux-gnu")
  build_for("x86_64-pc-windows-gnu")
  build_for("x86_64-apple-darwin")

  if "azure-angel" in str(platform.node()):
    print("Detected azure-angel, moving app builds for testing on other hosts...")
    subprocess.run(['sh', '-c',
      'cp target/x86_64-unknown-linux-gnu/release/meili /j/public/meili_linux ;'+
      'cp target/x86_64-pc-windows-gnu/release/meili.exe /j/public/meili_win.exe ;'+
      'cp target/x86_64-apple-darwin/release/meili /j/public/meili_macos'
    ]).check_returncode()

if __name__ == '__main__':
  main(sys.argv)


