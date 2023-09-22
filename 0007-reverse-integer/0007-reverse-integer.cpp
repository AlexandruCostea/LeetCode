class Solution {
public:
    int reverse(int x) {
        if(x == 0)
            return 0;
        int nr = abs(x);
        long long reverse=0;
        while(nr > 0) {
            reverse = reverse*10+nr%10;
            if(reverse > INT32_MAX)
                return 0;
            nr/=10;
        }
        return (int)reverse * (x/abs(x));
    }
};