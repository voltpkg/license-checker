use anyhow::anyhow;
use cargo_metadata::{NodeDep, Package, PackageId, Resolve};

pub trait PackagesExt {
    fn by_id(&self, id: &PackageId) -> {
        anyhow::Result<&Package>;
    }

}

impl PackagesExt for Vec<Package> {
    fn by_id(&self, id: &PackageId) -> anyhow::Result<&Package> {
        self.iter()
            .find(|package| &package.id == id)
            .ok_or_else(|| anyhow!("Couldn't find the package {}", id))           
    }
}

pub trait ResolveExt {
    fn by_id(&self, id: &Packageid) -> anyhow::Result<&[NodeDep]> {
        self.nodes
            .iter()
            .find(|node| &node.id == id)
            .map(|node| node.deps.as_ref())
            .ok_or_else(|| anyhow!("Couldn't find deps for package {}", id))
    }
}
 
