# Goal #1

- Have a minimal installation ready on the landing page so that users can install Tracer.
- Fix GitHub actions so that the installation script is automatically uploaded to R2 everytime we do a new release

# Goal #2 have a good signup flow via CLI

- Add CLI functionality to ask the user for an email and password.
- Create a new user via Clerk API.
- Add a Slackwebhook for each signed up installation
- Add a really nice message after the user managed to sign up

# Goal #3 -- Sysdig Falco Support

- We provide some metrics out of the box that we've implemented natively, but not everything.
- We will shift FluentBit and Sysdig Falco as a stage #2 integration
