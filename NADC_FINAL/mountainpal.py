# Dynamic programming Python implementation
# of LIS problem
 
import sys
# lis returns length of the longest
# increasing subsequence in arr of size n

def find_mountain_length(start, middle, end):
    side_length = min(middle - start, end - middle)
    return side_length * 2 + 1

 
def max_mountain_length(arr):
    n = len(arr)
 
    # (start_pos, increasing/decreasing, midpoint)
    # True for increasing, False for decreasing
    active_mountain = [0, True, 0]
    # Array of mountain lengths of the form [...mountain_i_length,...]
    # Negative 1 added for case when no mountains
    mountains = [-1]
 
    # Compute optimized LIS values in bottom up manner
    for i in range(1, n):
        # If active mountain is currently increasing
        if active_mountain[1]:
            if arr[i] < arr[i - 1]:
                if i > 1 and arr[i] == arr[i - 2]:
                    # Stop increasing, start decreasing
                    active_mountain[1] = False
                    active_mountain[2] = i - 1
                else:
                    # No palendrome, Kill active mountain, we're done
                    active_mountain[1] = i
            if arr[i] == arr[i - 1]:
                # We have flattop, kill active mountain
                active_mountain = [i, True, 0]
            # otherwise, do nothing, keep increasing

        # Mountain is currently decreasing
        else:
            if arr[i] > arr[i - 1]:
                # We stopped decreasing
                mountains.append(find_mountain_length(active_mountain[0], active_mountain[2], i - 1))
                active_mountain = [i - 1, True, 0]
            elif arr[i] != arr[active_mountain[2] - (i - active_mountain[2])]:
                # Don't worry about specificilly considering flat valley here because flat valley implies palendrome broken
                # We break palendrome, kill active mountain
                mountains.append(find_mountain_length(active_mountain[0], active_mountain[2], i - 1))
                active_mountain = [i, True, 0]
                continue
            elif i == n - 1:
                mountains.append(find_mountain_length(active_mountain[0], active_mountain[2], i))

            
    return max(mountains)
 
 
# Driver program to test above function
length = int(sys.stdin.readline())
arr = [1] * length

for i in range(length):
    arr[i] = int(sys.stdin.readline())


print(f"{max_mountain_length(arr)}")