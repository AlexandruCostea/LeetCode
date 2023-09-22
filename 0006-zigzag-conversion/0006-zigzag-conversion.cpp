class Solution {
public:
    string convert(string s, int numRows) {
        if(numRows == 1 || numRows > s.length())
            return s;
        vector<string> rows(numRows,"");
        int i = 0, j = 0, stringLength = s.length();
        while(j < stringLength) {
            while(i < numRows-1 && j < stringLength)
                rows[i++]+=s[j++];
            while(i > 0 && j < stringLength)
                rows[i--]+=s[j++];
        }
        string sol = "";
        for(i = 0; i < numRows; i++)
            sol+=rows[i];
        return sol;
    }
};