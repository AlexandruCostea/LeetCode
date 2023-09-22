class Solution {
    public int maxArea(int[] height) {
        int i=0,j=height.length-1,maxSize = 0,size,min;
        while(i < j) {
            min = (height[i] < height[j]) ? height[i] : height[j];
            size = min*(j-i);
            if(size > maxSize)
                maxSize = size;
            if(min == height[i])
                i++;
            else
                j--;
        }
        return maxSize;
    }
}