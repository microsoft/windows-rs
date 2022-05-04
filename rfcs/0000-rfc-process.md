- Feature Name: `rfc_process`
- Start Date: 2022-05-03
- RFC PR: [microsoft/windows-rs#0000](https://github.com/microsoft/windows-rs/pull/0000)
- Issue: [microsoft/windows-rs#0000](https://github.com/microsoft/windows-rs/issues/0000)

# Summary
[summary]: #summary

The "RFC" (request for comments) process is intended to provide a consistent and controlled path for new features to enter the windows-rs library ecosystem, so that all stakeholders can be confident about the direction the library is evolving in.

# Motivation
[motivation]: #motivation

The `windows` crate has experienced rapid change and experimentation in the past 2 years. This experimentation has been
invaluable, but it is important to start solidifying design. The outcome of following the RFC process should ultimately be:

- less churn - once features are integrated they typically change very little, especially to outside users
- features are generally of higher quality
- there is a shared design ownership stewarded by all windows-rs stakeholders

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

Introducing new features or new internal architecture to the windows-rs ecosystem requires creating an RFC (request for comments) which allows others to comment on and suggest improvements to proposals for changes.

An RFC should be opened when the following criteria are met:

- The author has a fully formed idea of the feature and can express in detail how it will work.
- There has been at least some discussion of the feature beforehand (i.e., the RFC should not be a surprise to contributors).
- The feature has far reaching consequences to users or contributors of the windows-rs ecosystem, and it would be difficult to reverse. This criterion should be applied more strongly for user facing changes than for internal architecture.
- There is potential for disagreement among contributors or users for how the change should be architected.

The process for opening an RFC is simple:

- Ensure there is an existing issue on the windows-rs repo where the feature has at least been discussed and there seems to be some consensus that an RFC would be appropriate.
- Fill out the [RFC template](./0000-template.md) with sufficient information to address all concerns.
- Engage in conversation on the RFC and revise it until concerns are addressed (either by changing the RFC or justifying why the concern is not relevant).
- Once conversation on the RFC has reached a steady state and all concerns have been addressed, windows-rs maintainers will either accept the RFC and merge it or provide a reason for why it should be closed.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

**Accepting an RFC**: an RFC will be accepted when there is no longer meaningful conversation left on the RFC, and all concerns have been sufficiently addressed. All core maintainers of the windows-rs ecosystem should be consulted on whether they believe the previous criteria has been met. Since maintainers of the RFC are also engaging in the meaningful conversation, there is no need for a written justification for why the RFC has been accepted.

# Drawbacks
[drawbacks]: #drawbacks

The RFC process require a certain amount of energy to maintain and also limits the amount of open experimentation that can be done in production releases. This ultimately means that development of windows-rs will slow some. However, this should be acceptable as the quality of features that do land will generally be much higher and the likelihood of needing to revert large changes will be much smaller.

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

The [RFC process](https://github.com/rust-lang/rfcs) has had proven success in the Rust community. While the Rust language RFC process is not perfect it has successfully ensured properties that we deem as desirable including stability, high-quality features, and wide consensus from many stakeholders.

As an alternative, we could continue to allow experimentation in code, discussing changes as PRs. However, such a process is generally much less rigorous than the RFC process and requires implementation work before ensuring that the design itself has wide support. Such a process is generally much more wasteful than trying to arrive on design consensus before implementation begins.

The other alternative is to continue to iterate by introducing changes, cutting releases, and gathering feedback. This process leads to a lot of churn and not only makes it hard to arrive at good designs, but also actively hinders our goals of working towards stabilization.

# Prior art
[prior-art]: #prior-art

As discussed, the Rust language has used the RFC process to great success, and most of the [criticisms](https://www.ncameron.org/blog/the-problem-with-rfcs/) of the process deal with problems that become much more apparent at scale. However, the goals for the windows-rs RFC process defined in the motivation section are very much achievable despite some of the drawbacks of the RFC process experienced by the Rust community.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- We may have to experiment with what constitutes a trivial change and what requires an RFC and how this differs for user facing changes vs internal changes.

# Future possibilities
[future-possibilities]: #future-possibilities

Much like the Rust project, we may in the future want to adopt additional processes around stabilization and lightweight process for internal changes (e.g. [major change proposals](https://github.com/rust-lang/rfcs/blob/master/text/2904-compiler-major-change-process.md)). Whatever we may wish to adopt in the future, it should be compatible with this RFC proposal.
