class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int maxLength = 0;
        queue<char> q;
        set<char> characters;
        for(auto c : s) {
            if(characters.count(c) == 0) {
                characters.insert(c);
                q.push(c);
            }
            else {
                if(q.size() > maxLength)
                    maxLength = q.size();
                while(q.front() != c){
                    characters.erase(q.front());
                    q.pop();
                }
                q.pop();
                q.push(c);
            }
        }
        if(q.size() > maxLength)
            maxLength = q.size();
        return maxLength;
    }
};