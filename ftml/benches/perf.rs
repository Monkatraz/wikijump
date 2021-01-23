/*
 * benches/perf.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(soft_unstable)]

//! Transient file to occassionally utilize to benchmark the library's performance.
//!
//! On a separate branch because this Rust feature requires nightly.

#[macro_use]
extern crate bencher;
extern crate ftml;
extern crate slog;

#[macro_use]
extern crate str_macro;

use bencher::Bencher;
use ftml::DebugIncluder;

macro_rules! build_logger {
    () => {
        slog::Logger::root(slog::Discard, slog::o!())
    };
}

fn full(bench: &mut Bencher) {
    let log = build_logger!();

    bench.iter(|| {
        let mut text = str!(INPUT);

        // Run includer
        ftml::include(&log, &mut text, DebugIncluder);

        // Run preprocessor
        ftml::preprocess(&log, &mut text);

        // Run lexer
        let tokens = ftml::tokenize(&log, &text);

        // Run parser
        let result = ftml::parse(&log, &tokens);
        let (_tree, _errors) = result.into();
    });
}

fn include(bench: &mut Bencher) {
    let log = build_logger!();

    bench.iter(|| {
        let mut text = str!(INPUT);

        // Run includer
        ftml::include(&log, &mut text, DebugIncluder);
    });
}

fn preprocess(bench: &mut Bencher) {
    let log = build_logger!();

    bench.iter(|| {
        let mut text = str!(INPUT);

        // Run preprocessor
        ftml::preprocess(&log, &mut text);
    });
}

fn tokenize(bench: &mut Bencher) {
    let log = build_logger!();

    let mut text = str!(INPUT);

    // Run preprocessor
    ftml::preprocess(&log, &mut text);

    bench.iter(|| {
        // Run lexer
        let _tokens = ftml::tokenize(&log, &text);
    });
}

fn parse(bench: &mut Bencher) {
    let log = build_logger!();

    let mut text = str!(INPUT);

    // Run preprocessor
    ftml::preprocess(&log, &mut text);

    // Run lexer
    let tokens = ftml::tokenize(&log, &text);

    bench.iter(|| {
        // Run parser
        let result = ftml::parse(&log, &tokens);
        let (_tree, _errors) = result.into();
    });
}

benchmark_group!(benches, full, preprocess, tokenize, parse);
benchmark_main!(benches);

// Test Data //

const INPUT: &str = r#"
[[include theme:black-highlighter-theme]]

[[>]]
[[module Rate]]
[[/>]]

[[include component:info-ayers
    lang=en |
    page=scp-4339 |
    authorPage=http://www.scp-wiki.net/aismallard |
    comments=
**SCP-4339:** The Attribute Pen
**Author:** [[*user aismallard]]

Image is an original creation by the author. CC-BY-SA 3.0 license.

= **Next:** [[[SCP-3597]]] - //Maladroit//
]]

[[include component:anomaly-class-bar-source
    item-number=4339 |
    clearance=4 |
    container-class=safe |
    secondary-class=none |
    disruption-class=ekhi |
    risk-class=caution
]]

----

[[include component:image-block
    name=scp-4339.jpeg |
    caption=SCP-4339
]]

**Special Containment Procedures:** SCP-4339 is to be stored in a maximum security small-item locker. Testing may not be conducted without the --approval of two Level-4 personnel-- unanimous consent of the O5 Council. (See Addendum 4339-2)

**Description:** SCP-4339 is a ball-point pen with no visible manufacturer markings. Laboratory testing has revealed no mind-altering or memetic effects resulting from exposure to the object.

SCP-4339 exhibits its anomalous properties when --written with-- held. (See Experiment 4339-04) When the current possessor makes a statement about the world, the item causes a reality restructuring event to make it true. This does not affect physical properties.

**Addendum 4339-1:** Experiment Logs
[[collapsible show="▸ Access File" hide="▾ Close File"]]
Experiments began 20██/05/10 09:13 and were overseen by Dr. Archibald.

> __**Experiment 4339-01**__
> **Procedure:** D-91386 was given a red plastic ball. Subject was asked to write "my ball is green".
> **Results:** D-91386 and the researchers present described the ball as "green". However, chromatic analysis reported that the ball did not change in color. Those present agreed with this assessment.

> __**Experiment 4339-02**__
> **Procedure:** D-91386 was asked if they had any emotional attachment to the ball. The subject replied in the negative. The subject was told to write "this ball is my favorite". D-10381, who had no previous contact with the subject or their ball, then entered the room.
> **Results:** D-10381 was asked if they thought the plastic ball was D-91386's favorite. They replied that they did not know. D-91386 was asked the same question. They answered in the affirmative.

> __**Experiment 4339-03**__
> **Procedure:** D-91386 was instructed to write "this ball belongs to D-10381".
> **Results:** Both D-class were asked who the ball belonged to. They both agreed it belonged to D-10381. D-91386 expressed minor regret over the loss of their favorite green ball.

> __**Experiment 4339-04**__
> **Procedure:** D-91386 was instructed to write "the ball was manufactured by Synthetic Colored Plastics Inc."
> **Results:** D-91386 said "my ball was stolen from me" while holding SCP-4339. D-10381 felt remorseful, and returned the ball to D-91386. D-91386 appeared happy. Researcher Chen reprimanded D-10381 for the theft.
> **Notes:** This test was performed unintentionally.

> __**Experiment 4339-11**__
> **Procedure:** D-91386 was told to say "my name is D-88111".
> **Results:** D-88111's identification was changed.

> __**Experiment 4339-14**__
> **Procedure:** D-41562 and D-34177 were brought into the testing room. D-88111 held SCP-4339 and pronounced D-41562 and D-34177 husband and wife. D-88111 is not an ordained priest or marriage officiant.
> **Results:** A local county official was asked about D-41562's marriage status. They confirmed that D-41562 and D-34177 were in fact legally married.
> **Addendum 20██/05/21:** After a complaint, the Ethics Committee liaison decided that D-41562/D-34177's requests for weekly conjugal visits may not be denied except as punishment for insubordination.

> __**Experiment 4339-27**__
> **Procedure:** D-88111 was instructed to create an LLC in the state of Virginia called "Specialized Corporate Properties".
> **Results:** D-88111 yawned and said they were exhausted, telling researchers that they were done testing for the day. They were dismissed.
> **Notes:** This test was performed unintentionally. Due to D-88111's tendency to speak without approval, it was decided they would be amnesticized and transferred to a different project.

> __**Experiment 4339-28**__
> **Procedure:** Junior Researcher Adams would take over for D-88111. They would use SCP-4339 to create the aforementioned LLC.
> **Results:** Upon touching SCP-4339, Junior Researcher Adams declared that the Equal Rights Amendment was part of the United States Constitution. They were disciplined for their unapproved use of an anomalous artifact.
> **Addendum:** US cable television aired extensive coverage of the newly added 28th amendment. Legal pundits were in agreement over its legitimacy, but were confused as to its origin.

> __**Experiment 4339-29**__
> **Procedure:** Dr. Archibald would assume control of SCP-4339. He would use it to restore political normalcy with minimal effect on the larger world. After some debate, the research team decided to retroactively create a standard Constitutional Convention to propose the amendment, and have it pass with a high degree of popular support.
> **Results:** National news agencies attributed the prior confusion to excitement.
> **Notes:** Dr. Archibald departed the testing chamber to take an official call from [REDACTED]. Testing was suspended until he returned.

> __**Experiment 4339-30**__
> **Procedure:** (No test was planned)
> **Results:** Dr. Archibald entered the testing chamber, seemingly distressed. He wrote on a piece of paper using SCP-4339. He did not permit anybody to see what was being written. Afterwards, while still in the possession of SCP-4339, O5-14 announced that he believed he had acted in a manner consistent with the goals and practices of the Foundation. O5-14 appeared to relax.
> **Addendum:** Due to the dangers inherent in the use of SCP-4339, O5-14 has cancelled all further testing.
[[/collapsible]]

**Addendum 4339-2:** Ethics Committee Moratorium
[[collapsible show="▸ Access File" hide="▾ Close File"]]
> **Ethics Committee Memo**
>
> **Date:** 20██/06/03
> **To:** Overseer Council
> **From:** Chairwoman Summers, Vice-Chairman Wendell
> **Cc:** Ethics Committee
> **Attn:** SCP-4339
>
> During our standard review of newly-created SCP files, an unusual attempted expungement of experiment logs was noticed. After having RAISA restore the original file, we have reviewed it and decided to scrutinize this matter further. This memorandum is to inform the Council that O5-14 is under investigation for potentially unethical utilization of SCP-4339.
>
> Effective immediately, all access to SCP-4339 is suspended and its file has been frozen. O5-14 is subject to mandatory recusal on all matters related to SCP-4339.
[[/collapsible]]

**Addendum 4339-3:** Overseer Debriefing
[[collapsible show="▸ Access File" hide="▾ Close File"]]
> **Task Force Action Report**
>
> **Relevant Forces:** [[[tanhony-s-proposal|Mobile Task Force Omega-1 ("Law's Left Hand")]]]
> **Ordering Body:** Ethics Committee
> **Location:** O5 Meeting Room, Site-01
> **Date:** 20██/06/17 16:31
>
> **Planned Actions:**
> * Detain O5-14.
> * Strip O5-14's Level-5 clearance.
> * Reorganize the Overseer Council to remove its 14th seat.
>
> **Resultant Actions:**
> O5-14 was successfully detained and demoted. Dr. Archibald is currently a Level-0 E-Class individual in Site-██ Detention Cell ███ awaiting sentencing.
>
> **Justification:**
> After a thorough investigation, complete with interviews with the Defendant and researchers involved in testing of SCP-4339, the Ethics Committee has decided that O5-14 violated the Foundation Code of Ethics egregiously enough to warrant removal from the Overseer Council. A full report is available to O5 members upon request.
>
> **Other Actions:**
> SCP-4339's special containment procedures were amended to require a unanimous O5 vote. Its file has been unfrozen, though the Council is advised to be cautious with future testing.
[[/collapsible]]

[[footnoteblock]]

[[div class="footer-wikiwalk-nav"]]
[[=]]
<< [[[SCP-4338]]] | SCP-4339 | [[[SCP-4340]]] >>
[[/=]]
[[/div]]

[[include :scp-sandbox-3:more-by-aismallard]]

[[include component:license-box]]
=====
> **Filename:** scp-4339.jpeg
> **Author:** [[*user aismallard]]
> **License:** CC-BY-SA 3.0
> **Source Link:** Self
=====
[[include component:license-box-end]]
"#;
