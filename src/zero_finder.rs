// Rough idea ->
// Build flat model simulation
// find Angles from .zero method
// create another builder from From impl
// mutate this builder with zero conditions
// create table from this

        // MySimulation(builder.solve_for(
        //     args.value_of("zero-distance")
        //         .unwrap_or("200")
        //         .parse()
        //         .unwrap(),
        //     args.value_of("zero-height").unwrap_or("0").parse().unwrap(),
        //     args.value_of("zero-offset").unwrap_or("0").parse().unwrap(),
        //     args.value_of("zero-tolerance")
        //         .unwrap_or("0.001")
        //         .parse()
        //         .unwrap(),
        //     args.value_of("pitch").unwrap_or("0").parse().unwrap(),
        //     args.value_of("yaw").unwrap_or("0").parse().unwrap(),
        // ))

//    fn solve_for(
//        &'a self,
//        zero_distance: Numeric,
//        zero_elevation_offset: Numeric,
//        zero_windage_offset: Numeric,
//        zero_tolerance: Numeric,
//        pitch_offset: Numeric,
//        yaw_offset: Numeric,
//    ) -> Simulation {
//        let zero_distance = Length::Yards(zero_distance);
//        let zero_elevation_offset = Length::Inches(zero_elevation_offset);
//        let zero_windage_offset = Length::Inches(zero_windage_offset);
//        let zero_tolerance = Length::Inches(zero_tolerance);
//        let pitch_offset = Angle::Minutes(pitch_offset);
//        let yaw_offset = Angle::Minutes(-yaw_offset); // Invert this number, since +90 is left in trig calculations
//
//        // Attempt to zero to given parameters, accounting for different conditions
//        // Start with 0.0 pitch and 0.0 yaw
//        // Then use found pitch/yaw for this simulation
//        let (found_pitch, found_yaw) = self
//            .using_zero_conditions(0.0, 0.0)
//            .zero(
//                zero_distance,
//                zero_elevation_offset,
//                zero_windage_offset,
//                zero_tolerance,
//            )
//            .map(|(found_pitch, found_yaw)| {
//                (
//                    Angle::Radians(
//                        found_pitch.to_radians().to_num() + pitch_offset.to_radians().to_num(),
//                    ),
//                    Angle::Radians(
//                        found_yaw.to_radians().to_num() + yaw_offset.to_radians().to_num(),
//                    ),
//                )
//            })
//            .expect("solve_for");
//
//        Simulation::new(
//            &self.flags,
//            &self.projectile,
//            &self.scope,
//            &self.solve_conditions,
//            self.time_step,
//            found_pitch,
//            found_yaw,
//        )
//    }


        // .zero_conditions(
        //     Conditions::new()
        //         .with_temperature(
        //             args.value_of("zero-temperature").unwrap_or("68").parse().unwrap())
        //             .expect("zero-temperature")
        //         .with_pressure(
        //             args.value_of("zero-pressure").unwrap_or("29.92").parse().unwrap())
        //             .expect("zero-pressure")
        //         .with_humidity(
        //             args.value_of("zero-humidity").unwrap_or("0").parse().unwrap())
        //             .expect("zero-humidity")
        //         .with_wind_speed(
        //             args.value_of("zero-wind-speed").unwrap_or("0").parse().unwrap())
        //             .expect("zero-wind-speed")
        //         .with_wind_angle(
        //             args.value_of("zero-wind-angle").unwrap_or("0").parse().unwrap())
        //             .expect("zero-wind-angle")
        //         .with_shot_angle(
        //             args.value_of("zero-shot-angle").unwrap_or("0").parse().unwrap())
        //             .expect("zero-shot-angle")
        //         .with_lattitude(
        //             args.value_of("zero-lattitude").unwrap_or("0").parse().unwrap())
        //             .expect("zero-lattitude")
        //         .with_bearing(
        //             args.value_of("zero-bearing").unwrap_or("0").parse().unwrap())
        //             .expect("zero-bearing")
        //         .with_gravity(
        //             args.value_of("zero-gravity").unwrap_or("-32.174").parse().unwrap())
        // )
