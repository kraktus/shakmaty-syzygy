#!/usr/local/bin/python3
#coding: utf-8
# Licence: GNU AGPLv3

""""""

from __future__ import annotations

import os
import subprocess

#############
# Constants #
#############

########
# Logs #
########

###########
# Classes #
###########

class Req:

    def __init__(self) -> None:
        http = requests.Session()
        http.mount("https://", ADAPTER)
        http.mount("http://", ADAPTER)
        self.http = http


def main() -> None:
	for nb_piece in ["SOURCE-5.txt", "SOURCE-6.txt"]:
		for url in open(f"tables/chess/{nb_piece}", "r"):
			url = url.strip()
			file_name = url.split('/')[-1]
			if ".rtbz" in file_name:
				continue
			print(file_name)
			cmd = ["/usr/bin/curl", "-r", "0-207", url]
			ouput = subprocess.run(cmd, check=True, capture_output=True)
			with open(f"partial_dl/{file_name}", "wb") as f:
				f.write(ouput.stdout)

########
# Main #
########

if __name__ == "__main__":
    print('#'*80)
    main()