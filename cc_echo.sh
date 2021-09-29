#!/bin/bash

cc tmp.s -o tmp && ./tmp
echo "result:$?"