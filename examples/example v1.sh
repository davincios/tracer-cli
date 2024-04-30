# Set up the API key once for all future sessions or until changed
tracer setup HyBmo-zcUzRBn2crJDVPf

# Start a new tracing session, preparing the environment
tracer start

# Run a tool like STAR with specified version and flags, using defaults intelligently
tracer tool STAR 2.7.11b
tracer log --type warning "QC mapping reads GC content below 21% threshold"

# End the current tracing session, finalizing all tasks
tracer end

# A general help command that provides a comprehensive overview of all available commands and options
tracer help
