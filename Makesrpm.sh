# 1. Ensure the directory structure exists
rpmdev-setuptree

# 2. Create the Source Tarball manually
# CRITICAL: The filename MUST match 'Source0' in your .spec file exactly.
# If your spec says "Rusty-0.1.0.tar.gz", the file must be named exactly that.
tar -czf ~/rpmbuild/SOURCES/Rusty-0.1.0.tar.gz .

# Verify the file exists
ls -l ~/rpmbuild/SOURCES/Rusty-0.1.0.tar.gz

# 3. Generate the SRPM
# This reads the .spec file and the tarball you just created
rpmbuild -bs Rusty.spec
