class Solution {
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        int left = 0, right=0, sum = 0, minLength = nums.size()+1;
        while(right < nums.size()) {
            sum+=nums[right];
            while(sum >= target) {
                minLength = min(right-left+1, minLength);
                sum-=nums[left];
                left++;
            }
            right++;
        }
        return (minLength < nums.size()+1) ? minLength : 0;
    }
};