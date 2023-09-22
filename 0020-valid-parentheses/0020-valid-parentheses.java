class Solution {
        public boolean isValid(String s) {
        Stack<Character> stack = new Stack<>();
        char[] arr = s.toCharArray();
        for(char c: arr) {
            if(c == '(' || c =='[' || c == '{') {
                stack.add(c);
            }
            else {
                if(!stack.isEmpty()) {
                    char currentCharacter = stack.pop();
                    if ((currentCharacter == '(' && c != ')') || (currentCharacter == '[' && c != ']')
                            || (currentCharacter == '{' && c != '}')) {
                        return false;
                    }
                }
                else {
                    return false;
                }
            }
        }
        return stack.isEmpty();
    }
}