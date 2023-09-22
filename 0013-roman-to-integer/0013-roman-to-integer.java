class Solution {
        public int romanToInt(String s) {
       String currentDigit= new String("");
       int number=0,i=0;
       char[] romanNumber = s.toCharArray();
       while(i<romanNumber.length && romanNumber[i] == 'M') {
            currentDigit+='M';
            i++;
       }
       number+=1000*currentDigit.length();
       currentDigit="";
       while(i<romanNumber.length && (romanNumber[i] == 'M' || romanNumber[i] == 'D' ||
        romanNumber[i] == 'C')) {
            currentDigit+=romanNumber[i];
            i++;
        }
        switch (currentDigit) {
            case "CM":
                number+=900;
                break;
            case "DCCC":
                number+=800;
                break;
            case "DCC":
                number+=700;
                break;
            case "DC":
                number+=600;
                break;
            case "D":
                number+=500;
                break;
            case "CD":
                number+=400;
                break;
            case "CCC":
                number+=300;
                break;
            case "CC":
                number+=200;
                break;
            case "C":
                number+=100;
                break;
        }
        currentDigit="";
        while(i<romanNumber.length && (romanNumber[i] == 'C' || romanNumber[i] == 'L' ||
        romanNumber[i] == 'X')) {
            currentDigit+=romanNumber[i];
            i++;
        }
        switch (currentDigit) {
            case "XC":
                number+=90;
                break;
            case "LXXX":
                number+=80;
                break;
            case "LXX":
                number+=70;
                break;
            case "LX":
                number+=60;
                break;
            case "L":
                number+=50;
                break;
            case "XL":
                number+=40;
                break;
            case "XXX":
                number+=30;
                break;
            case "XX":
                number+=20;
                break;
            case "X":
                number+=10;
                break;
        }
        currentDigit="";
        while(i < romanNumber.length && (romanNumber[i] == 'X' || romanNumber[i] == 'V' ||
        romanNumber[i] == 'I')) {
            currentDigit+=romanNumber[i];
            i++;
        }
        switch (currentDigit) {
            case "IX":
                number+=9;
                break;
            case "VIII":
                number+=8;
                break;
            case "VII":
                number+=7;
                break;
            case "VI":
                number+=6;
                break;
            case "V":
                number+=5;
                break;
            case "IV":
                number+=4;
                break;
            case "III":
                number+=3;
                break;
            case "II":
                number+=2;
                break;
            case "I":
                number+=1;
                break;
        }
        return number;
    }
}