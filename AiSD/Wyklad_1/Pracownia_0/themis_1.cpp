#include<iostream>
using namespace std;
/// Marcin Czapulak
/// 324279
/// ATA
/// 
int main()
{
    long long a,b,acc=0;
    cin>>a>>b;
    while(acc <= b)
    {
        if (acc >= a)
        {
            cout<<acc;
            if (acc + 2024 < b)
            {
                cout<<" ";
            }
        }
        acc+=2024;
    }
    return 0;
}