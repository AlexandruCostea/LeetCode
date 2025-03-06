class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> sol;
        sort(nums.begin(), nums.end());
        int prev = nums[0]-1;
        for(int i = 0; i < nums.size() && nums[i] <= 0; i++) {
            if(nums[i] != prev) {
                for(int j = i+1; j < nums.size(); j++) {
                    int left = j+1, right = nums.size()-1, mid;
                    while(left <= right && nums[left] <= 0-nums[i]-nums[j]) {
                        mid = (left+right)/2;
                        if(nums[mid] == 0-nums[i]-nums[j]) {
                            vector<int> v {nums[i],nums[j],nums[mid]};
                            if(find(sol.begin(), sol.end(), v) == sol.end())
                                sol.push_back(v);
                            break;
                        }
                        else if (nums[mid] > 0-nums[i]-nums[j])
                            right = mid-1;
                        else
                            left = mid+1;
                    }
                }
            }
            prev = nums[i];
        }
        return sol;
    }
};