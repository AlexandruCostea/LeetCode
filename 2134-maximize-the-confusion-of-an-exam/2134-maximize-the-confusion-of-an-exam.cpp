class Solution {
public:
    int maxConsecutiveAnswers(string answerKey, int k) {
        int minCount,maxSize = 0;
        unordered_map<char,int> charCount;
        for(int right = 0; right < answerKey.length(); right++) {
            charCount[answerKey[right]]++;
            minCount = min(charCount['T'],charCount['F']);
            if(minCount <= k)
                maxSize++;
            else {
                charCount[answerKey[right-maxSize]]--;
            }
        }
        return maxSize;
    }
};