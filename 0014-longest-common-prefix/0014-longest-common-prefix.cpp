class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        int k = 0;
        while(k < strs[0].length()) {
            for(int i = 1; i < strs.size(); i++) {
                if(k >= strs[i].size())
                    return strs[i];
                if(strs[i][k] != strs[0][k]) {
                    strs[0].erase(k, strs[0].length()-k);
                    return strs[0];
                }
            }
            k++;
        }
        return strs[0];
    }
};