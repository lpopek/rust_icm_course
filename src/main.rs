// P1
//recuretial function
fn collatz_stopping(c0:usize, n:usize, max:usize) -> usize{
    let cn;
    if n == max{
        return c0;
    }
    println!("current number: c{} = {}", n, c0);
    if c0 % 2 == 0{
        cn = c0/2;
        //println!("Decrease number");
        collatz_stopping(cn, n + 1, max)
        
    }
    else{
        cn = 3 * c0 + 1;
        //println!("Increase number");
        collatz_stopping(cn, n + 1, max)
    }

}
// with loop
fn collatz_stopping_loop(c0:usize, max:usize) ->usize{
    let mut cn = c0;
    for i in 0..max {
        println!("current number: c{} = {}", i, cn);
        if cn % 2 == 0{
            cn = cn/2;
            println!("Decrease number");
        }
        else{
            cn = 3 * cn + 1;
            println!("Increase number");
        } // x: i32
    }
    return cn;
}

//P2
fn choose(n:usize,k:usize)->usize{
    if k > n{
        return 0;
    }
    if k == 1{
        return n;
    }
    else{
        return choose(n-1, k-1) + choose(n-1, k)
    }
}

//P3
fn e_value()->f64{
    let mut i = 0;
    let mut sum = 1.0_f64;
    let mut last_fact = 1;
    loop{
        i += 1;
        if ((1.0_f64).exp() - sum) < 10e-6{
            return sum;
        }
        else {
            last_fact *= i;
            let component = 1.0_f64/last_fact as f64;
            sum += component;
            // println!("Current value of e: {}", sum);
        }
    }
}
//P6
fn eu_gcd(a:usize,b:usize)->usize{
    if b == 0{
        return a;
    }
    else{
        eu_gcd(b, a % b)
    }
}


fn main() {
    //println!("Collatz stopping C10 recurential: {}", collatz_stopping(27, 0, 111));
    //println!("Collatz stopping C10 recurential with loop: {}", collatz_stopping_loop(27, 111));
    println!("Binomial theorem result for n = 10 and k = 5: {}", choose(10, 5));
    println!("Value of e calculated from formula: {}", e_value());
    println!("GCD from 85 and 102 is: {}", eu_gcd(102, 85));
}
