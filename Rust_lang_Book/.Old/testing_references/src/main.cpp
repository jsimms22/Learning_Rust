#include <iostream>

using namespace std;

int main(int argc, char **argv)
{
    int s = 5;

    printf("printing variable 's': ");
    printf("%d\n",s);

    printf("printing 's' by reference: ");
    printf("%d\n",&s);

    int *s_ptr = &s;

    printf("printing 's_ptr': ");
    printf("%d\n",s_ptr);

    printf("printing 's_ptr' by reference: ");
    printf("%d\n",&s_ptr);

    printf("printing '*s_ptr': ");
    printf("%d\n",*s_ptr);

    return 0;
} 
