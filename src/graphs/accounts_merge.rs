use std::collections::HashMap;

// Union Find over emails.
//
// Treat each unique email as a node. Emails that appear in the same account
// belong to the same person, so union every email in an account with the first
// email of that account.
//
// `email_to_id` maps each email to its DSU id. The same id is also the index
// into `emails` and `names`.
//
// After all unions, each DSU component is one merged account. Group emails by
// their root, sort each group, and prepend the account name.
//
// Time: O(E * α(U) + U log U), where E is total email entries and U is the number
// of unique emails. Sorting the grouped emails dominates in practice.
// Space: O(U), for DSU, email/name storage, and grouped output.

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut dsu = Dsu::new();

    let mut email_to_id = HashMap::new();
    let mut emails = Vec::new();
    let mut names = Vec::new();

    for account in &accounts {
        let name = &account[0];

        let first_id = get_or_add_email(
            &account[1],
            name,
            &mut email_to_id,
            &mut emails,
            &mut names,
            &mut dsu,
        );

        for email in &account[2..] {
            let id = get_or_add_email(
                email,
                name,
                &mut email_to_id,
                &mut emails,
                &mut names,
                &mut dsu,
            );

            dsu.union(first_id, id);
        }
    }

    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();

    for id in 0..emails.len() {
        let root = dsu.find(id);
        groups.entry(root).or_default().push(emails[id].clone());
    }

    let mut res = Vec::with_capacity(groups.len());

    for (root, mut group) in groups {
        group.sort_unstable();

        let mut account = Vec::with_capacity(group.len() + 1);
        account.push(names[root].clone());
        account.extend(group);

        res.push(account);
    }

    res
}

// Return the existing id for an email, or create a new DSU node for it.
// The id is the shared index into DSU, `emails`, and `names`.
fn get_or_add_email(
    email: &str,
    name: &str,
    email_to_id: &mut HashMap<String, usize>,
    emails: &mut Vec<String>,
    names: &mut Vec<String>,
    dsu: &mut Dsu,
) -> usize {
    if let Some(&id) = email_to_id.get(email) {
        return id;
    }

    let id = dsu.add();
    email_to_id.insert(email.to_string(), id);
    emails.push(email.to_string());
    names.push(name.to_string());

    id
}

#[derive(Default)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self) -> usize {
        let Self { parent, size } = self;
        let id = parent.len();
        parent.push(id);
        size.push(1);

        id
    }

    fn find(&mut self, node: usize) -> usize {
        let Self { parent, .. } = self;
        let mut cur = node;

        while cur != parent[cur] {
            parent[cur] = parent[parent[cur]];
            cur = parent[cur];
        }

        cur
    }

    fn union(&mut self, node1: usize, node2: usize) -> bool {
        let par1 = self.find(node1);
        let par2 = self.find(node2);

        if par1 == par2 {
            return false;
        }

        let Self { parent, size } = self;

        if size[par1] > size[par2] {
            parent[par2] = par1;
            size[par1] += size[par2];
        } else {
            parent[par1] = par2;
            size[par2] += size[par1];
        }

        true
    }
}
