# Spire

A script scheduler, task graph processor, and evaluator of such.

## Uses

Spire is intended to serve as a scheduler similar to `cron`, but with some
niceties of task-graph processing and other triggers added in. In a way, it's
intended to be a lot like [Concourse CI](https://concourse-ci.org/), just in a
way that is more oriented towards manual use and being customized.

Things spire can do:

- Run jobs. This sounds a bit trivial (you could just do it by hand in a shell),
  but `spire-engine` offers an API on a port, meaning that with something like
  Tailscale there's now a generic "run this thing" environment for your stuff.
- Run jobs _but on a schedule_. It's `cron`, with the same benefits as running a
  single job.
- Run _graphs_ of jobs. This is where we get a bit closer to Airflow and the
  likes, but yeah, same idea.

If this sounds a lot like Concourse CI to you, yeah, me too, except this one's
mine and it's a lot less of a pain in the ass to run as it is aimed at having
one user per setup.

This idea is vaguely inspired by [bank
python](https://calpaterson.com/bank-python.html), a hugely integrated stack of
software that is very different from the compartmentalised and distinct pieces
that people normally deal with.

## Architecture

Spire has two parts:

- `spire-engine`: the core scheduler and job runner that Does Stuff™. This is
  usable via a locally hosted webserver that accepts a vaguely standard API
  thing.
- `spire-cli`: a CLI that optimises for UX, so is less nuts and bolts.

`spire-engine` offering an API means that using something like Tailscale to
securely export ports on your machines now means you have a general-purpose
scheduler.

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

Stuff Spire definitely does *not* want to be:

- Scalable
- Universal
- Catering to a broad audience.

Think of it like you do of Netflix OSS software: It's public, you can
contribute patches, you can read the source code, you can raise bugs, but it's
most definitely not written for you, or anyone but netflix themselves.
