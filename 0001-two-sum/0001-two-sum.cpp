class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> map;
        vector<int> sol;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            int complement = target - nums[i];
            if (map.find(complement) != map.end()) {
                sol.push_back(map[complement]);
                sol.push_back(i);
                return sol;
            }
            map[nums[i]] = i;
        }
        return sol;
    }
};