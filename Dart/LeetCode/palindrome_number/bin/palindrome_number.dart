import 'dart:math';
class Solution {
  int numLen(int n) {
    int len = 0;

    while (n > 0) {
      n ~/= 10;
      len += 1;
    }

    return len;
  }

  bool isPalindrome(int x) {
    if (x < 0) {
      return false;
    }

    int len = this.numLen(x);
    int? left;
    int? right;
    while(len > 1) {
      left = x ~/ pow(10, len - 1);
      int right = x % 10;

      print("$len $left $right");

      if(left != right){
        return false;
      }

      x -= left * pow(10, len - 1) as int;
      x ~/= 10;
      len -= 2;
    }

    return true;
  }
}

void main(List<String> arguments) {
  Solution s = Solution();

  print(s.isPalindrome(1004554001));
}
