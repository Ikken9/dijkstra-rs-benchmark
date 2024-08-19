use criterion::{criterion_group, criterion_main, Criterion};
use dijkstra_rs::{Edge, Graph, Vertex, VertexId};

pub fn bench_dijkstra_heap(c: &mut Criterion) {
    let edge_a_b = Edge { to: VertexId('b'), weight: 2 };
    let edge_b_a = Edge { to: VertexId('a'), weight: 2 };
    let edge_b_d = Edge { to: VertexId('d'), weight: 5 };
    let edge_d_b = Edge { to: VertexId('b'), weight: 5 };
    let edge_c_d = Edge { to: VertexId('d'), weight: 8 };
    let edge_d_c = Edge { to: VertexId('c'), weight: 8 };
    let edge_d_g = Edge { to: VertexId('g'), weight: 4 };
    let edge_e_g = Edge { to: VertexId('g'), weight: 2 };
    let edge_a_c = Edge { to: VertexId('c'), weight: 2 };
    let edge_b_c = Edge { to: VertexId('c'), weight: 1 };
    let edge_c_a = Edge { to: VertexId('a'), weight: 2 };
    let edge_d_e = Edge { to: VertexId('e'), weight: 17 };
    let edge_e_d = Edge { to: VertexId('d'), weight: 8 };
    let edge_e_f = Edge { to: VertexId('f'), weight: 11 };
    let edge_f_e = Edge { to: VertexId('e'), weight: 10 };
    let edge_f_g = Edge { to: VertexId('g'), weight: 3 };
    let edge_g_f = Edge { to: VertexId('f'), weight: 4 };
    let edge_g_h = Edge { to: VertexId('h'), weight: 13 };
    let edge_h_g = Edge { to: VertexId('g'), weight: 15 };
    let edge_h_i = Edge { to: VertexId('i'), weight: 26 };
    let edge_i_h = Edge { to: VertexId('h'), weight: 7 };
    let edge_i_j = Edge { to: VertexId('j'), weight: 9 };
    let edge_j_i = Edge { to: VertexId('i'), weight: 33 };
    let edge_j_k = Edge { to: VertexId('k'), weight: 16 };
    let edge_k_j = Edge { to: VertexId('j'), weight: 25 };
    let edge_k_l = Edge { to: VertexId('l'), weight: 30 };
    let edge_l_k = Edge { to: VertexId('k'), weight: 11 };
    let edge_l_m = Edge { to: VertexId('m'), weight: 11 };
    let edge_m_l = Edge { to: VertexId('l'), weight: 51 };
    let edge_m_n = Edge { to: VertexId('n'), weight: 11 };
    let edge_n_m = Edge { to: VertexId('m'), weight: 21 };
    let edge_n_o = Edge { to: VertexId('o'), weight: 31 };
    let edge_o_n = Edge { to: VertexId('n'), weight: 41 };
    let edge_o_p = Edge { to: VertexId('p'), weight: 51 };
    let edge_p_o = Edge { to: VertexId('o'), weight: 61 };
    let edge_p_q = Edge { to: VertexId('q'), weight: 15 };
    let edge_q_p = Edge { to: VertexId('p'), weight: 14 };
    let edge_q_r = Edge { to: VertexId('r'), weight: 18 };
    let edge_r_q = Edge { to: VertexId('q'), weight: 19 };
    let edge_r_s = Edge { to: VertexId('s'), weight: 18 };
    let edge_s_r = Edge { to: VertexId('r'), weight: 10 };
    let edge_s_t = Edge { to: VertexId('t'), weight: 10 };
    let edge_t_s = Edge { to: VertexId('s'), weight: 10 };
    let edge_t_u = Edge { to: VertexId('u'), weight: 10 };
    let edge_u_t = Edge { to: VertexId('t'), weight: 10 };
    let edge_u_v = Edge { to: VertexId('v'), weight: 10 };
    let edge_v_u = Edge { to: VertexId('u'), weight: 10 };
    let edge_v_w = Edge { to: VertexId('w'), weight: 10 };
    let edge_w_v = Edge { to: VertexId('v'), weight: 10 };
    let edge_w_x = Edge { to: VertexId('x'), weight: 112 };
    let edge_x_w = Edge { to: VertexId('w'), weight: 11 };
    let edge_x_y = Edge { to: VertexId('y'), weight: 31 };
    let edge_y_x = Edge { to: VertexId('x'), weight: 15 };
    let edge_y_z = Edge { to: VertexId('z'), weight: 1 };
    let edge_z_y = Edge { to: VertexId('y'), weight: 1 };

    let vertex_a = Vertex { id: VertexId('a'), edges: vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a] };
    let vertex_b = Vertex { id: VertexId('b'), edges: vec![edge_b_a, edge_a_b, edge_b_d, edge_d_b] };
    let vertex_c = Vertex { id: VertexId('c'), edges: vec![edge_c_a, edge_a_c, edge_c_d, edge_d_c] };
    let vertex_d = Vertex { id: VertexId('d'), edges: vec![edge_d_b, edge_b_d, edge_d_c, edge_c_d, edge_d_e, edge_d_g] };
    let vertex_e = Vertex { id: VertexId('e'), edges: vec![edge_e_f, edge_e_g, edge_b_c] };
    let vertex_f = Vertex { id: VertexId('f'), edges: vec![edge_f_e, edge_g_h] };
    let vertex_g = Vertex { id: VertexId('g'), edges: vec![edge_g_f, edge_f_g] };
    let vertex_h = Vertex { id: VertexId('h'), edges: vec![edge_h_g, edge_h_i] };
    let vertex_i = Vertex { id: VertexId('i'), edges: vec![edge_i_h, edge_i_j] };
    let vertex_j = Vertex { id: VertexId('j'), edges: vec![edge_j_i, edge_j_k] };
    let vertex_k = Vertex { id: VertexId('k'), edges: vec![edge_k_j, edge_k_l] };
    let vertex_l = Vertex { id: VertexId('l'), edges: vec![edge_l_k, edge_l_m] };
    let vertex_m = Vertex { id: VertexId('m'), edges: vec![edge_m_l, edge_m_n] };
    let vertex_n = Vertex { id: VertexId('n'), edges: vec![edge_n_m, edge_n_o] };
    let vertex_o = Vertex { id: VertexId('o'), edges: vec![edge_o_n, edge_o_p] };
    let vertex_p = Vertex { id: VertexId('p'), edges: vec![edge_p_o, edge_p_q] };
    let vertex_q = Vertex { id: VertexId('q'), edges: vec![edge_q_p, edge_q_r] };
    let vertex_r = Vertex { id: VertexId('r'), edges: vec![edge_r_q, edge_r_s] };
    let vertex_s = Vertex { id: VertexId('s'), edges: vec![edge_s_r, edge_s_t] };
    let vertex_t = Vertex { id: VertexId('t'), edges: vec![edge_t_s, edge_t_u] };
    let vertex_u = Vertex { id: VertexId('u'), edges: vec![edge_u_t, edge_u_v] };
    let vertex_v = Vertex { id: VertexId('v'), edges: vec![edge_v_u, edge_v_w] };
    let vertex_w = Vertex { id: VertexId('w'), edges: vec![edge_w_v, edge_w_x] };
    let vertex_x = Vertex { id: VertexId('x'), edges: vec![edge_x_w, edge_x_y] };
    let vertex_y = Vertex { id: VertexId('y'), edges: vec![edge_y_x, edge_y_z] };
    let vertex_z = Vertex { id: VertexId('z'), edges: vec![edge_z_y, edge_e_d] };

    let mut graph = Graph::new();
    graph.add_vertex(vertex_a.clone());
    graph.add_vertex(vertex_b);
    graph.add_vertex(vertex_c);
    graph.add_vertex(vertex_d);
    graph.add_vertex(vertex_e);
    graph.add_vertex(vertex_f);
    graph.add_vertex(vertex_g);
    graph.add_vertex(vertex_h);
    graph.add_vertex(vertex_i);
    graph.add_vertex(vertex_j);
    graph.add_vertex(vertex_k);
    graph.add_vertex(vertex_l);
    graph.add_vertex(vertex_m);
    graph.add_vertex(vertex_n);
    graph.add_vertex(vertex_o);
    graph.add_vertex(vertex_p);
    graph.add_vertex(vertex_q);
    graph.add_vertex(vertex_r);
    graph.add_vertex(vertex_s);
    graph.add_vertex(vertex_t);
    graph.add_vertex(vertex_u);
    graph.add_vertex(vertex_v);
    graph.add_vertex(vertex_w);
    graph.add_vertex(vertex_x);
    graph.add_vertex(vertex_y);
    graph.add_vertex(vertex_z);

    c.bench_function("dijkstra_heap", |b| {
        b.iter(|| {
            std::hint::black_box( {
                graph.dijkstra_heap(vertex_a.clone())
            });
        });
    });
}

pub fn bench_dijkstra_no_heap(c: &mut Criterion) {
    let edge_a_b = Edge { to: VertexId('b'), weight: 2 };
    let edge_b_a = Edge { to: VertexId('a'), weight: 2 };
    let edge_b_d = Edge { to: VertexId('d'), weight: 5 };
    let edge_d_b = Edge { to: VertexId('b'), weight: 5 };
    let edge_c_d = Edge { to: VertexId('d'), weight: 8 };
    let edge_d_c = Edge { to: VertexId('c'), weight: 8 };
    let edge_d_g = Edge { to: VertexId('g'), weight: 4 };
    let edge_e_g = Edge { to: VertexId('g'), weight: 2 };
    let edge_a_c = Edge { to: VertexId('c'), weight: 2 };
    let edge_b_c = Edge { to: VertexId('c'), weight: 1 };
    let edge_c_a = Edge { to: VertexId('a'), weight: 2 };
    let edge_d_e = Edge { to: VertexId('e'), weight: 17 };
    let edge_e_d = Edge { to: VertexId('d'), weight: 8 };
    let edge_e_f = Edge { to: VertexId('f'), weight: 11 };
    let edge_f_e = Edge { to: VertexId('e'), weight: 10 };
    let edge_f_g = Edge { to: VertexId('g'), weight: 3 };
    let edge_g_f = Edge { to: VertexId('f'), weight: 4 };
    let edge_g_h = Edge { to: VertexId('h'), weight: 13 };
    let edge_h_g = Edge { to: VertexId('g'), weight: 15 };
    let edge_h_i = Edge { to: VertexId('i'), weight: 26 };
    let edge_i_h = Edge { to: VertexId('h'), weight: 7 };
    let edge_i_j = Edge { to: VertexId('j'), weight: 9 };
    let edge_j_i = Edge { to: VertexId('i'), weight: 33 };
    let edge_j_k = Edge { to: VertexId('k'), weight: 16 };
    let edge_k_j = Edge { to: VertexId('j'), weight: 25 };
    let edge_k_l = Edge { to: VertexId('l'), weight: 30 };
    let edge_l_k = Edge { to: VertexId('k'), weight: 11 };
    let edge_l_m = Edge { to: VertexId('m'), weight: 11 };
    let edge_m_l = Edge { to: VertexId('l'), weight: 51 };
    let edge_m_n = Edge { to: VertexId('n'), weight: 11 };
    let edge_n_m = Edge { to: VertexId('m'), weight: 21 };
    let edge_n_o = Edge { to: VertexId('o'), weight: 31 };
    let edge_o_n = Edge { to: VertexId('n'), weight: 41 };
    let edge_o_p = Edge { to: VertexId('p'), weight: 51 };
    let edge_p_o = Edge { to: VertexId('o'), weight: 61 };
    let edge_p_q = Edge { to: VertexId('q'), weight: 15 };
    let edge_q_p = Edge { to: VertexId('p'), weight: 14 };
    let edge_q_r = Edge { to: VertexId('r'), weight: 18 };
    let edge_r_q = Edge { to: VertexId('q'), weight: 19 };
    let edge_r_s = Edge { to: VertexId('s'), weight: 18 };
    let edge_s_r = Edge { to: VertexId('r'), weight: 10 };
    let edge_s_t = Edge { to: VertexId('t'), weight: 10 };
    let edge_t_s = Edge { to: VertexId('s'), weight: 10 };
    let edge_t_u = Edge { to: VertexId('u'), weight: 10 };
    let edge_u_t = Edge { to: VertexId('t'), weight: 10 };
    let edge_u_v = Edge { to: VertexId('v'), weight: 10 };
    let edge_v_u = Edge { to: VertexId('u'), weight: 10 };
    let edge_v_w = Edge { to: VertexId('w'), weight: 10 };
    let edge_w_v = Edge { to: VertexId('v'), weight: 10 };
    let edge_w_x = Edge { to: VertexId('x'), weight: 112 };
    let edge_x_w = Edge { to: VertexId('w'), weight: 11 };
    let edge_x_y = Edge { to: VertexId('y'), weight: 31 };
    let edge_y_x = Edge { to: VertexId('x'), weight: 15 };
    let edge_y_z = Edge { to: VertexId('z'), weight: 1 };
    let edge_z_y = Edge { to: VertexId('y'), weight: 1 };

    let vertex_a = Vertex { id: VertexId('a'), edges: vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a] };
    let vertex_b = Vertex { id: VertexId('b'), edges: vec![edge_b_a, edge_a_b, edge_b_d, edge_d_b] };
    let vertex_c = Vertex { id: VertexId('c'), edges: vec![edge_c_a, edge_a_c, edge_c_d, edge_d_c] };
    let vertex_d = Vertex { id: VertexId('d'), edges: vec![edge_d_b, edge_b_d, edge_d_c, edge_c_d, edge_d_e, edge_d_g] };
    let vertex_e = Vertex { id: VertexId('e'), edges: vec![edge_e_f, edge_e_g, edge_b_c] };
    let vertex_f = Vertex { id: VertexId('f'), edges: vec![edge_f_e, edge_g_h] };
    let vertex_g = Vertex { id: VertexId('g'), edges: vec![edge_g_f, edge_f_g] };
    let vertex_h = Vertex { id: VertexId('h'), edges: vec![edge_h_g, edge_h_i] };
    let vertex_i = Vertex { id: VertexId('i'), edges: vec![edge_i_h, edge_i_j] };
    let vertex_j = Vertex { id: VertexId('j'), edges: vec![edge_j_i, edge_j_k] };
    let vertex_k = Vertex { id: VertexId('k'), edges: vec![edge_k_j, edge_k_l] };
    let vertex_l = Vertex { id: VertexId('l'), edges: vec![edge_l_k, edge_l_m] };
    let vertex_m = Vertex { id: VertexId('m'), edges: vec![edge_m_l, edge_m_n] };
    let vertex_n = Vertex { id: VertexId('n'), edges: vec![edge_n_m, edge_n_o] };
    let vertex_o = Vertex { id: VertexId('o'), edges: vec![edge_o_n, edge_o_p] };
    let vertex_p = Vertex { id: VertexId('p'), edges: vec![edge_p_o, edge_p_q] };
    let vertex_q = Vertex { id: VertexId('q'), edges: vec![edge_q_p, edge_q_r] };
    let vertex_r = Vertex { id: VertexId('r'), edges: vec![edge_r_q, edge_r_s] };
    let vertex_s = Vertex { id: VertexId('s'), edges: vec![edge_s_r, edge_s_t] };
    let vertex_t = Vertex { id: VertexId('t'), edges: vec![edge_t_s, edge_t_u] };
    let vertex_u = Vertex { id: VertexId('u'), edges: vec![edge_u_t, edge_u_v] };
    let vertex_v = Vertex { id: VertexId('v'), edges: vec![edge_v_u, edge_v_w] };
    let vertex_w = Vertex { id: VertexId('w'), edges: vec![edge_w_v, edge_w_x] };
    let vertex_x = Vertex { id: VertexId('x'), edges: vec![edge_x_w, edge_x_y] };
    let vertex_y = Vertex { id: VertexId('y'), edges: vec![edge_y_x, edge_y_z] };
    let vertex_z = Vertex { id: VertexId('z'), edges: vec![edge_z_y, edge_e_d] };

    let mut graph = Graph::new();
    graph.add_vertex(vertex_a.clone());
    graph.add_vertex(vertex_b);
    graph.add_vertex(vertex_c);
    graph.add_vertex(vertex_d);
    graph.add_vertex(vertex_e);
    graph.add_vertex(vertex_f);
    graph.add_vertex(vertex_g);
    graph.add_vertex(vertex_h);
    graph.add_vertex(vertex_i);
    graph.add_vertex(vertex_j);
    graph.add_vertex(vertex_k);
    graph.add_vertex(vertex_l);
    graph.add_vertex(vertex_m);
    graph.add_vertex(vertex_n);
    graph.add_vertex(vertex_o);
    graph.add_vertex(vertex_p);
    graph.add_vertex(vertex_q);
    graph.add_vertex(vertex_r);
    graph.add_vertex(vertex_s);
    graph.add_vertex(vertex_t);
    graph.add_vertex(vertex_u);
    graph.add_vertex(vertex_v);
    graph.add_vertex(vertex_w);
    graph.add_vertex(vertex_x);
    graph.add_vertex(vertex_y);
    graph.add_vertex(vertex_z);

    c.bench_function("dijkstra_no_heap", |b| {
        b.iter(|| {
            std::hint::black_box( {
                graph.dijkstra_no_heap(vertex_a.clone())
            });
        });
    });
}

criterion_group!(
    benches,
    bench_dijkstra_heap,
    bench_dijkstra_no_heap
);

criterion_main!(benches);