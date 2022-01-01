# Munnin

Or, my version of [huginn](https://github.com/huginn/huginn). It is, in
essence, a script scheduler, task graph processor, and evaluator of such.

## Architecture

Munnin is written as a classic daemon, that meaning, a headless process that
sits in the background and Does Stuff. There's then various ways of interacting
with the daemon.

## Interface

Munnin plans to support two interfaces:

- A CLI that communicates with the daemon, and
- an API that also communicates with the daemon.

## Why write this?

The reasons are varied, but boil down to some subset of this:

- Having my own project for this allows me to build stuff as I want,
- I need it to be portable between OSX and Linux,
- I want to understand how these things work, and writing one seems like a great
  exercise in doing so.
- Having a system under my control for exactly this purpose allows me to
  integrate my own tools into it in a way that would make no sense for any other
  public tool to offer as an API.

### Anti-goals

Stuff Munnin definitely does *not* want to be:

- Scalable
- Universal
- Catering to a broad audience.

Think of it like you do of Netflix OSS software: It's public, you can
contribute patches, you can read the source code, you can raise bugs, but it's
most definitely not written for you, or anyone but netflix themselves.
