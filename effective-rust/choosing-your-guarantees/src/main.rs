fn main() {
    {
        use std::cell::Cell;

        let x = Cell::new(1);
        let y = &x;
        let z = &x;
        x.set(2);
        y.set(3);
        z.set(4);
        println!("{}", x.get());
    }

    {
        use std::cell::RefCell;

        let x = RefCell::new(vec![1, 2, 3, 4]);
        {
            println!("{:?}", *x.borrow());
        }
        {
            let mut my_ref = x.borrow_mut();
            my_ref.push(1);
        }
    }
}
