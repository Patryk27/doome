use super::*;

pub fn serialize(lbvh: LinearBvh) -> (GeometryIndex, usize) {
    let mut out = Vec::new();

    for node in lbvh {
        let v1;
        let v2;

        match node {
            LinearBvhNode::Leaf { triangle, goto_id } => {
                let goto_ptr = goto_id.map(|id| id * 2).unwrap_or_default();

                v1 = vec4(0.0, 0.0, 0.0, triangle.get() as _);
                v2 = vec4(0.0, 0.0, 0.0, goto_ptr as _);
            }

            LinearBvhNode::NonLeaf {
                bb,
                on_hit_goto_id,
                on_miss_goto_id,
            } => {
                let on_hit_goto_ptr =
                    on_hit_goto_id.map(|id| id * 2).unwrap_or_default();

                let on_miss_goto_ptr =
                    on_miss_goto_id.map(|id| id * 2).unwrap_or_default();

                v1 = bb.min().extend(on_hit_goto_ptr as _);
                v2 = bb.max().extend(on_miss_goto_ptr as _);
            }
        }

        out.push(v1);
        out.push(v2);
    }

    let out_len = out.len();

    // ----

    while out.len() < 4096 {
        out.push(vec4(0.0, 0.0, 0.0, 0.0));
    }

    let out = out.try_into().unwrap_or_else(|out: Vec<_>| {
        panic!(
            "ayy ayy the geometry index is too large -- produced {} items",
            out.len()
        );
    });

    (GeometryIndex::new(out), out_len)
}
