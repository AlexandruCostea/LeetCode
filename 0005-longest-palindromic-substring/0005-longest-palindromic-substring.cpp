class Solution {
public:
    string longestPalindrome(string s) {
        int left = 0, right,auxLeft,auxRight,ok,found;
        string palindrome = "";
        while(left < s.length()) {
            right = s.length()-1;
            found = 0;
            while(!found) {
                while(s[right] != s[left])
                    right--;
                ok = 1;
                auxLeft = left;
                auxRight = right;
                while(ok && auxLeft < auxRight) {
                    if(s[auxLeft] != s[auxRight])
                        ok=0;
                    auxLeft++;
                    auxRight--;
                }
                if(ok) {
                    found = 1;
                    if(right-left+1 > palindrome.length()) {
                        palindrome = "";
                        for(int i = left; i <= right; i++)
                            palindrome+=s[i];
                    }
                }
                else
                    right--;
            }
            left++;
        }
        return palindrome;
    }
};