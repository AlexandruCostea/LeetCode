class Solution {
public:
    void generateSolutions(string& currentSolution, int level, 
    int target,const vector<char>& options, vector<string>& solutions) {
        for(char option : options) {
            currentSolution[level] = option;
            bool valid = true;
            int openCount = 0, closedCount = 0;
            for(int i = 0; i <= level; i++) {
                if(currentSolution[i] == '(')
                    openCount++;
                else
                    closedCount++;
                if(closedCount > openCount) {
                    valid = false;
                    break;
                }
            }
            if(valid) {
                if(level == target) {
                    if(openCount == closedCount)
                        solutions.push_back(currentSolution);
                }
                else
                    generateSolutions(currentSolution,level+1,target,options,solutions);
            }
        }
    }

    vector<string> generateParenthesis(int n) {
        vector<string> solutions;
        string currentSolution(n*2, ' ');
        vector<char> options = {'(', ')'};
        generateSolutions(currentSolution, 0, n*2-1,options, solutions);
        return solutions;
    }
};