#!/usr/bin/env python3
# -*- coding: utf-8; py-indent-offset:4 -*-
"""sad

Command line tool that takes a binary data string from Solana
account along with a yaml file describing the data and
generates a deserialized output"""

import cmdline
from borsh_construct import U8, String, CStruct

animal = CStruct(
    "dname" / String,
    "legs" / U8,
)

print(animal.__dict__)
ser_animal = animal.build({"dname": "Francis", "legs": 15})
print(ser_animal)
deser_animal = animal.parse(ser_animal)
print(f"Name {deser_animal.dname}")
print(deser_animal)

s = String.build("Francis")
print(s)
sd = String.parse(s)
print(sd)
