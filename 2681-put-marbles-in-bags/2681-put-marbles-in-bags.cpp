class Solution {
public:
    long long putMarbles(vector<int>& weights, int k) {
        if(weights.size() <= 2)
            return 0;
        vector<long long> pairs;
        long long sol = 0, n = weights.size();
        for(int i = 0; i < n -1; i++)
            pairs.push_back(weights[i] + weights[i+1]);
        sort(pairs.begin(), pairs.end());
        for(int i = 0; i < k-1; i++) {
            sol+=pairs[n-2-i]-pairs[i];
        }
        return sol;
    }
};