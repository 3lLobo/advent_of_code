# For each day in each year add a build and test to the rust.yml
# empty array
export years=()
export days=()

for file in *; do
  if [ -d $file ]; then
    years+=($file)
  fi
done

for year in "${years[@]}"; do
  for file in $year/*; do
    # Find folders which start with day
    if [[ $file == *day* ]]; then
      days+=($file)
    fi
  done
done

export RUSTYML="./.github/workflows/rust.yml"
# Write the new workflow file
echo "name: Rust" >$RUSTYML
echo "on: [push, pull_request]" >>$RUSTYML
echo "jobs:" >>$RUSTYML
echo "  build:" >>$RUSTYML
echo "    runs-on: ubuntu-latest" >>$RUSTYML
echo "    steps:" >>$RUSTYML
echo "      - uses: actions/checkout@v2" >>$RUSTYML

# Add a build and test for each day
for day in "${days[@]}"; do
  echo "      - name: Build $day" >>$RUSTYML
  echo "        run: cd $day && cargo build" >>$RUSTYML
  echo "      - name: Test $day" >>$RUSTYML
  echo "        run: cd $day && cargo test" >>$RUSTYML
done
