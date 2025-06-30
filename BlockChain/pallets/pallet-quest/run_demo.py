#!/usr/bin/env python3
"""
Script to run the Activities and Zoologist's Lodge demo.
"""

import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

# Import the demo function
from demo import run_demo

if __name__ == "__main__":
    run_demo()