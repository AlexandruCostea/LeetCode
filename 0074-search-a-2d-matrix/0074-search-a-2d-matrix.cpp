class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int i = 0,j;
        while(i < matrix.size() && matrix[i].at(matrix[i].size()-1) < target)
            i++;
        if(i == matrix.size())
            return false;
        for(j = 0; j < matrix[i].size(); j++)
            if(matrix[i].at(j) == target)
                return true;
        return false;
    }
};