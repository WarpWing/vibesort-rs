#!/usr/bin/env python3
import time
from vibesort import vibesort

def test_vibesort():
    # Test with numbers
    numbers = [42, 7, 13, 99, 1, 56]
    print(f"Original numbers: {numbers}")
    
    start_time = time.time()
    sorted_numbers = vibesort(numbers)
    end_time = time.time()
    
    print(f"Sorted numbers: {sorted_numbers}")
    print(f"Time taken: {(end_time - start_time) * 1000:.2f}ms")
    print()
    
    # Test with different numbers
    more_numbers = [87, 23, 45, 12, 67, 34, 89, 5]
    print(f"Original numbers: {more_numbers}")
    
    start_time = time.time()
    sorted_more = vibesort(more_numbers)
    end_time = time.time()
    
    print(f"Sorted numbers: {sorted_more}")
    print(f"Time taken: {(end_time - start_time) * 1000:.2f}ms")
    print()
    
    # Test with single digit numbers
    single_digits = [9, 2, 7, 1, 4, 8, 3, 6, 5]
    print(f"Original numbers: {single_digits}")
    
    start_time = time.time()
    sorted_singles = vibesort(single_digits)
    end_time = time.time()
    
    print(f"Sorted numbers: {sorted_singles}")
    print(f"Time taken: {(end_time - start_time) * 1000:.2f}ms")

if __name__ == "__main__":
    test_vibesort()
