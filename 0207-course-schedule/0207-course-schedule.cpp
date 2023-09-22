class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<list<int>> graph(numCourses);
        vector<int> indegree(numCourses, 0);
        for(int i = 0; i < prerequisites.size(); i++) {
            graph[prerequisites[i][1]].push_front(prerequisites[i][0]);
            indegree[prerequisites[i][0]]++;
        }
        list<int> topo;
        queue<int> q;
        for(int i = 0; i < indegree.size(); i++)
            if(indegree[i] == 0)
                q.push(i);
        while(!q.empty()) {
            int node = q.front();
            q.pop();
            topo.push_front(node);
            for(int x : graph[node]) {
                indegree[x]--;
                if(indegree[x] == 0)
                    q.push(x);
            }
        }
        return topo.size() == numCourses;
    }
};