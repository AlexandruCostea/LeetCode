class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        int subarray1Size = 0, subArray2Size = 0, zeroCount = 0,prev = -1, maxSize = 0,hasZero=0;
        for(int i = 0; i < nums.size(); i++) {
            if(nums[i] == 1) {
                if(zeroCount == 0)
                    subarray1Size++;
                else
                    subArray2Size++;
            }
            else {
                if(zeroCount == 0) {
                    hasZero = 1;
                    if(prev != -1 && prev != 0)
                        zeroCount++;
                }
                else {
                    if(maxSize < subarray1Size + subArray2Size)
                        maxSize = subarray1Size + subArray2Size;
                    zeroCount = 1;
                    subarray1Size = subArray2Size;
                    subArray2Size = 0;
                }
            }
            prev = nums[i];
        }
        if(hasZero == 0)
            return nums.size()-1;
        if(maxSize < subarray1Size + subArray2Size)
            maxSize = subarray1Size + subArray2Size;
        return maxSize;
    }
};