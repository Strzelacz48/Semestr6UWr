#include <iostream>
#include <vector>
#include <algorithm>
#include <math.h>

using namespace std;

typedef struct result {
    double perimeter;
    pair<int,int> p1;
    pair<int,int> p2;
    pair<int,int> p3;
}Result;

// Obliczanie odleglosci euklidesowej
double dist(const pair<int, int>& a, const pair<int, int>& b) { 
    double x = a.first - b.first;
    double y = a.second - b.second;
    return sqrt( x*x + y*y ); 
}

Result find_triangle(vector < pair<int,int> > &vectorX, int beg, int end){
    if ((end - beg) < 3){
        return Result{INFINITY, make_pair(0,0), make_pair(0,0), make_pair(0,0)};
    }
    else if ((end - beg) == 3){
        double sum = dist(vectorX[beg], vectorX[beg+1]) + dist(vectorX[beg], vectorX[beg+2]) + dist(vectorX[beg+1], vectorX[beg+2]);
        return Result{sum, vectorX[beg], vectorX[beg+1], vectorX[beg+2]};
    }
    
    Result left = find_triangle(vectorX, beg, (end + beg)/2);
    Result right = find_triangle(vectorX, (end + beg)/2, end);
    Result min_result = left.perimeter < right.perimeter ? left : right;

    vector<pair<int,int>> vectorY;
    for (int i = beg; i < end; i++){
        if (abs(vectorX[i].first - vectorX[(end + beg)/2].first) < min_result.perimeter/2){
            vectorY.push_back(vectorX[i]);
        }
    }
    sort(vectorY.begin(), vectorY.end(), [](pair<int,int> a, pair<int,int> b) { return a.second < b.second; });
    int size = vectorY.size();
    for (int i = 0; i < size - 2; i++){
        for (int j = i+1; j < size - 1; j++){
            if (abs(vectorY[i].second - vectorY[j].second) >= min_result.perimeter/2){
                break;
            }
            for (int k = j+1; k < size; k++){
                if (abs(vectorY[i].second - vectorY[k].second) >= min_result.perimeter/2 || abs(vectorY[j].second - vectorY[k].second) >= min_result.perimeter/2){
                    break;
                }
                double sum = dist(vectorY[i], vectorY[j]) + dist(vectorY[i], vectorY[k]) + dist(vectorY[j], vectorY[k]);
                if (sum < min_result.perimeter){
                    min_result = Result{sum, vectorY[i], vectorY[j], vectorY[k]};
                }
            }
        }
    }
    return min_result;
}

int main(){
    int n;
    cin >> n;
    vector < pair<int,int> > vectorX(n);
    for(int i = 0; i < n; i++){
        int x, y;
        cin >> x >> y;
        vectorX[i] = (make_pair(x,y));
    }
    sort(vectorX.begin(), vectorX.end(), [](pair<int,int> a, pair<int,int> b) { return a.first < b.first; });

    Result res = find_triangle(vectorX,0, n);
    cout<<res.p1.first<<" "<<res.p1.second<<endl;
    cout<<res.p2.first<<" "<<res.p2.second<<endl;
    cout<<res.p3.first<<" "<<res.p3.second<<endl;
    //realease memory
    vectorX.clear();
    return 0;
}