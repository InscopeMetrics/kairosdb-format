KairosDb Format
===========================

<a href="https://raw.githubusercontent.com/InscopeMetrics/kairosdb-format/master/LICENSE">
    <img src="https://img.shields.io/hexpm/l/plug.svg"
         alt="License: Apache 2">
</a>
<a href='https://build.arpnetworking.com/job/InscopeMetrics/job/kairosdb-format/job/master/'>
    <img src='https://build.arpnetworking.com/job/InscopeMetrics/job/kairosdb-format/job/master/badge/icon'
         alt='Jenkins Build'>
</a>
<a href="http://search.maven.org/#search%7Cga%7C1%7Cg%3A%22io.inscopemetrics.kairosdb%22%20a%3A%22format%22">
    <img src="https://img.shields.io/maven-central/v/io.inscopemetrics.kairosdb/format.svg"
         alt="Maven Artifact">
</a>

Defines a format for persisting augmented histograms in KairosDb using Protocol Buffers.

Features
--------

Stores a histogram and supporting statistics for recombinable queries of time series data using [KairosDb](https://kairosdb.github.io/).
Histogram bins are computed using floating point mantissa truncation to a specified precision. Error is fixed as a percentage of the value
and bins can be downsampled to a lesser precision without introducing any additional error. Supporting statistics for min, max, sum,
average and count (which is inferred from the histogram) are represented separately for accuracy and are also recombinable. The logic for
leveraging this data format is provided by [Kairosdb Extensions](https://github.com/InscopeMetrics/kairosdb-extensions).

Building
--------

Prerequisites:
* _None_

Building:

    kairosdb-format> ./jdk-wrapper.sh ./mvnw verify

To use the local version you must first install it locally:

    kairosdb-format> ./jdk-wrapper.sh ./mvnw install

You can determine the version of the local build from the pom file.  Using the local version is intended only for testing or development.

You may also need to add the local repository to your build in order to pick-up the local version:

* Maven - Included by default.
* Gradle - Add *mavenLocal()* to *build.gradle* in the *repositories* block.
* SBT - Add *resolvers += Resolver.mavenLocal* into *project/plugins.sbt*.

License
-------

Published under Apache Software License 2.0, see LICENSE

&copy; Dropbox, 2020
