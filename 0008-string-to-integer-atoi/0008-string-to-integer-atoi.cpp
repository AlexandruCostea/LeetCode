class Solution {
public:
    int myAtoi(string s) {
        int i = 0,sign = 1,n=s.length();
        long long nr = 0;
        while(s[i] == ' ')
            i++;
        if(i < n && s[i] == '-')
            sign*=-1;
        if(i < n && s[i] == '-' || s[i] == '+')
            i++;
        while(i < n && s[i] >= '0' && s[i] <= '9') {
            if((nr*10 + s[i]-'0') * sign > INT32_MAX) {
                return INT32_MAX;
            }
            if((nr*10 + s[i]-'0') * sign < INT32_MIN) {
                return INT32_MIN;
            }
            nr = nr*10+ s[i]-'0';
            i++;
        }
        nr*=sign;
        return (int)nr;
    }
};