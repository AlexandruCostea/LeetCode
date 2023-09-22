class Solution {
public:
    void generateSolutions(string& currentSol, const string& digits, int level, vector<string>& sol,
    unordered_map<char,vector<char>>& values) {
        char c = digits[level];
        for(char letter: values[c]) {
            currentSol[level] = letter;
            if(level == digits.size()-1) {
                sol.push_back(currentSol);
            }
            else
                generateSolutions(currentSol,digits,level+1,sol,values);
        }
    }

    vector<string> letterCombinations(string digits) {
        vector<string> sol;
        unordered_map<char,vector<char>> values;
        values['2'] = {'a','b','c'};
        values['3'] = {'d','e','f'};
        values['4'] = {'g','h','i'};
        values['5'] = {'j','k','l'};
        values['6'] = {'m','n','o'};
        values['7'] = {'p','q','r','s'};
        values['8'] = {'t','u','v'};
        values['9'] = {'w','x','y','z'};
        string currentSol = string(digits.size(), ' ');
        generateSolutions(currentSol, digits, 0, sol,values);
        return sol;
    }
};