 #include <stdio.h>

 extern int printResult(double result);

 int printResult(double result){
     printf("result = %f\n", result);
     return 0;
 }
