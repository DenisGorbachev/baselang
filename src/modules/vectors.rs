use crate::{Nats, Top, VarRc, exp, module, typ, var};

module!(
    pub struct Vectors {
        vector,
        nil,
        cons,
        append,
        append_nil,
        append_cons,
    }
);

pub type VectorsTuple = (VarRc, VarRc, VarRc, VarRc, VarRc, VarRc);

impl Vectors {
    pub fn new(nats: &Nats) -> Self {
        let Nats {
            nat,
            zero,
            next,
            add,
            ..
        } = nats;

        // Vector : (t : Top) -> (len : Nat) -> Top
        var!(t: Top);
        var!(len: typ!(exp!(nat)));
        var!(vector: typ!(t => typ!(len => Top)));

        // Nil : (t : Top) -> Vector t Zero
        let vector_t = exp!(&vector, &t);
        let vector_t_zero = exp!(vector_t.clone(), zero);
        var!(nil: typ!(t => typ!(vector_t_zero)));

        // Cons : (t : Top) -> (len : Nat) -> (head : t) -> (tail : Vector t len) -> Vector t (Next len)
        var!(head: typ!(exp!(t)));
        let vector_t_len = exp!(vector_t.clone(), &len);
        var!(tail: typ!(vector_t_len.clone()));
        let next_len = exp!(next, &len);
        let vector_t_next_len = exp!(vector_t.clone(), next_len);
        var!(cons: typ!(t => typ!(len => typ!(head => typ!(tail => typ!(vector_t_next_len))))));

        // Append : (t : Top) -> (len_a : Nat) -> (len_b : Nat) -> (a : Vector t len_a) -> (b : Vector t len_b) -> Vector t (Add len_a len_b)
        var!(len_a: typ!(exp!(nat)));
        var!(len_b: typ!(exp!(nat)));
        let vector_t_len_a = exp!(vector_t.clone(), &len_a);
        let vector_t_len_b = exp!(vector_t.clone(), &len_b);
        var!(a: typ!(vector_t_len_a.clone()));
        var!(b: typ!(vector_t_len_b.clone()));
        let add_len_a_len_b = exp!(add, &len_a, &len_b);
        let vector_t_add_len_a_len_b = exp!(vector_t.clone(), add_len_a_len_b);
        var!(append: typ!(t => typ!(len_a => typ!(len_b => typ!(a => typ!(b => typ!(vector_t_add_len_a_len_b)))))));

        // Append.Nil : (t : Top) -> (len_b : Nat) -> (b : Vector t len_b) -> (Append t Zero len_b (Nil t) b -> b)
        let nil_t = exp!(&nil, &t);
        let append_zero_len_b_nil_t_b_exp = exp!(&append, &t, zero, &len_b, nil_t, &b);
        var!(append_zero_len_b_nil_t_b: typ!(append_zero_len_b_nil_t_b_exp));
        var!(append_nil: typ!(t => typ!(len_b => typ!(b => typ!(append_zero_len_b_nil_t_b => typ!(&b))))));

        // Append.Cons : (t : Top) -> (len_a : Nat) -> (len_b : Nat) -> (head : t) -> (tail : Vector t len_a) -> (b : Vector t len_b) -> (Append t (Next len_a) len_b (Cons t len_a head tail) b -> Cons t (Add len_a len_b) head (Append t len_a len_b tail b))
        var!(head: typ!(exp!(t)));
        var!(tail: typ!(vector_t_len_a));
        let next_len_a = exp!(next, &len_a);
        let cons_t_len_a_head_tail = exp!(&cons, &t, &len_a, &head, &tail);
        let append_t_next_len_a_len_b_cons_b_exp = exp!(&append, &t, next_len_a, &len_b, cons_t_len_a_head_tail, &b);
        var!(append_t_next_len_a_len_b_cons_b: typ!(append_t_next_len_a_len_b_cons_b_exp));
        let add_len_a_len_b = exp!(add, &len_a, &len_b);
        let append_t_len_a_len_b_tail_b = exp!(&append, &t, &len_a, &len_b, &tail, &b);
        let cons_t_add_len_a_len_b_head_append = exp!(&cons, &t, add_len_a_len_b, &head, append_t_len_a_len_b_tail_b);
        var!(append_cons: typ!(t => typ!(len_a => typ!(len_b => typ!(head => typ!(tail => typ!(b => typ!(append_t_next_len_a_len_b_cons_b => typ!(cons_t_add_len_a_len_b_head_append)))))))));

        Self {
            vector,
            nil,
            cons,
            append,
            append_nil,
            append_cons,
        }
    }
}
