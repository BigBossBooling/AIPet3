#!/usr/bin/env python3
"""
Script to run the Activities and Zoologist's Lodge test.
"""

import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

if __name__ == "__main__":
    # Import the test functions only when script is run directly
    from run_test import test_activities, test_lodge
    print("Running Activities and Zoologist's Lodge Test...")
    test_activities()
    test_lodge()
    print("\nTest Complete!")