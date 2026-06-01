# Annex N: Annex N

## Annex N
(normative)
Algorithm for probabilistic distribution functions
N.1 General
This annex lists the C source code for the SystemVerilog probabilistic distribution system functions.
TableN.1 shows the SystemVerilog system function names with their corresponding Cfunctions. See 20.14
for the syntactical definition of these system functions.
Table N.1—SystemVerilog to C function cross-listing
SystemVerilog function C function
$dist_uniform rtl_dist_uniform
$dist_normal rtl_dist_normal
$dist_exponential rtl_dist_exponential
$dist_poisson rtl_dist_poisson
$dist_chi_square rtl_dist_chi_square
$dist_t rtl_dist_t
$dist_erlang rtl_dist_erlang
$random rtl_dist_uniform (seed, LONG_MIN, LONG_MAX)
The algorithm for these functions is defined by the C code in N.2.
N.2 Source code
/*
* Algorithm for probabilistic distribution functions.
*
* IEEE Std 1800-2023 SystemVerilog Unified Hardware Design and Verification Language
*/
#include <limits.h>
static double uniform( long *seed, long start, long end );
static double normal( long *seed, long mean, long deviation);
static double exponential( long *seed, long mean);
static long poisson( long *seed, long mean);
static double chi_square( long *seed, long deg_of_free);
static double t( long *seed, long deg_of_free);
static double erlangian( long *seed, long k, long mean);
long
rtl_dist_chi_square( seed, df )
long *seed;

<!-- Page 1339 -->

IEEE Std long df;
{
double r;
long i;
if(df>0)
{
r=chi_square(seed,df);
if(r>=0)
{
i=(long)(r+0.5);
}
else
{
r = -r;
i=(long)(r+0.5);
i = -i;
}
}
else
{
print_error("WARNING: Chi_square distribution must ",
"have positive degree of freedom\n");
i=0;
}
return (i);
}
long
rtl_dist_erlang( seed, k, mean )
long *seed;
long k, mean;
{
double r;
long i;
if(k>0)
{
r=erlangian(seed,k,mean);
if(r>=0)
{
i=(long)(r+0.5);
}
else
{
r = -r;
i=(long)(r+0.5);
i = -i;
}
}
else
{
print_error("WARNING: k-stage erlangian distribution ",
"must have positive k\n");
i=0;
}
return (i);
}

<!-- Page 1340 -->

IEEE Std long
rtl_dist_exponential( seed, mean )
long *seed;
long mean;
{
double r;
long i;
if(mean>0)
{
r=exponential(seed,mean);
if(r>=0)
{
i=(long)(r+0.5);
}
else
{
r = -r;
i=(long)(r+0.5);
i = -i;
}
}
else
{
print_error("WARNING: Exponential distribution must ",
"have a positive mean\n");
i=0;
}
return (i);
}
long
rtl_dist_normal( seed, mean, sd )
long *seed;
long mean, sd;
{
double r;
long i;
r=normal(seed,mean,sd);
if(r>=0)
{
i=(long)(r+0.5);
}
else
{
r = -r;
i=(long)(r+0.5);
i = -i;
}
return (i);
}
long
rtl_dist_poisson( seed, mean )

<!-- Page 1341 -->

IEEE Std long *seed;
long mean;
{
long i;
if(mean>0)
{
i=poisson(seed,mean);
}
else
{
print_error("WARNING: Poisson distribution must have a ",
"positive mean\n");
i=0;
}
return (i);
}
long
rtl_dist_t( seed, df )
long *seed;
long df;
{
double r;
long i;
if(df>0)
{
r=t(seed,df);
if(r>=0)
{
i=(long)(r+0.5);
}
else
{
r = -r;
i=(long)(r+0.5);
i = -i;
}
}
else
{
print_error("WARNING: t distribution must have positive ",
"degree of freedom\n");
i=0;
}
return (i);
}
long
rtl_dist_uniform(seed, start, end)
long *seed;
long start, end;
{
double r;
long i;
if (start >= end) return(start);
if (end != LONG_MAX)
{

<!-- Page 1342 -->

IEEE Std end++;
r = uniform( seed, start, end );
if (r >= 0)
{
i = (long) r;
}
else
{
i = (long) (r-1);
}
if (i<start) i = start;
if (i>=end) i = end-1;
}
else if (start!=LONG_MIN)
{
start--;
r = uniform( seed, start, end) + 1.0;
if (r>=0)
{
i = (long) r;
}
else
{
i = (long) (r-1);
}
if (i<=start) i = start+1;
if (i>end) i = end;
}
else
{
r =(uniform(seed,start,end)+
2147483648.0)/4294967295.0;
r = r*4294967296.0-2147483648.0;
if (r>=0)
{
i = (long) r;
}
else
{
i = (long) (r-1);
}
}
return (i);
}
static double
uniform( seed, start, end )
long *seed, start, end;
{
union u_s
{
float s;
unsigned stemp;
} u;
double d = 0.00000011920928955078125;
double a,b,c;

<!-- Page 1343 -->

IEEE Std if ((*seed) == 0)
*seed = 259341593;
if (start >= end)
{
a = 0.0;
b = 2147483647.0;
}
else
{
a = (double) start;
b = (double) end;
}
*seed = 69069 * (*seed) + 1;
u.stemp = *seed;
/*
* This relies on IEEE floating-point format
*/
u.stemp = (u.stemp >> 9) | 0x3f800000;
c = (double) u.s;
c = c+(c*d);
c = ((b - a) * (c - 1.0)) + a;
return (c);
}
static double
normal(seed,mean,deviation)
long *seed,mean,deviation;
{
double v1,v2,s;
double log(), sqrt();
s = 1.0;
while((s >= 1.0) || (s == 0.0))
{
v1 = uniform(seed,-1,1);
v2 = uniform(seed,-1,1);
s = v1 * v1 + v2 * v2;
}
s = v1 * sqrt(-2.0 * log(s) / s);
v1 = (double) deviation;
v2 = (double) mean;
return(s * v1 + v2);
}
static double
exponential(seed,mean)
long *seed,mean;
{
double log(),n;
n = uniform(seed,0,1);
if(n != 0)
{
n = -log(n) * mean;

<!-- Page 1344 -->

IEEE Std }
return(n);
}
static long
poisson(seed,mean)
long *seed,mean;
{
long n;
double p,q;
double exp();
n = 0;
q = -(double)mean;
p = exp(q);
q = uniform(seed,0,1);
while(p < q)
{
n++;
q = uniform(seed,0,1) * q;
}
return(n);
}
static double
chi_square(seed,deg_of_free)
long *seed,deg_of_free;
{
double x;
long k;
if(deg_of_free % 2)
{
x = normal(seed,0,1);
x = x * x;
}
else
{
x = 0.0;
}
for(k = 2; k <= deg_of_free; k = k + 2)
{
x = x + 2 * exponential(seed,1);
}
return(x);
}
static double
t(seed,deg_of_free)
long *seed,deg_of_free;
{
double sqrt(),x;
double chi2 = chi_square(seed,deg_of_free);
double div = chi2 / (double)deg_of_free;
double root = sqrt(div);
x = normal(seed,0,1) / root;
return(x);
}
static double
erlangian(seed,k,mean)

<!-- Page 1345 -->

IEEE Std long *seed,k,mean;
{
double x,log(),a,b;
long i;
x=1.0;
for(i=1;i<=k;i++)
{
x = x * uniform(seed,0,1);
}
a=(double)mean;
b=(double)k;
x= -a*log(x)/b;
return(x);
}

<!-- Page 1346 -->

IEEE Std 1800-2023