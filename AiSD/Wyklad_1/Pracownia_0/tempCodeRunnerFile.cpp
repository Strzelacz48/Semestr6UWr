#include<iostream>
using namespace std;
/// Marcin Czapulak
/// 324279
/// ATA
/// 
int main()
{
    long long a,b,acc=2024;
    cin>>a>>b;
    while(acc < b)
    {
        if (acc > a)
        {
            cout<<acc;
        }
        acc+=2024;
        if (acc <= b)
        {
            cout<<" ";
        }
    }
    return 0;
}