use crate::baca::api::baca_service::BacaApi;
use crate::command::Command;
use crate::error::Result;
use crate::model::Submit;
use crate::workspace::Workspace;

use clap::ArgMatches;
use tracing::info;

pub struct Details {
    submit_id: String,
}

impl From<&ArgMatches<'_>> for Details {
    fn from(args: &ArgMatches) -> Self {
        let submit_id = args.value_of("id").unwrap();
        Self {
            submit_id: submit_id.to_string(),
        }
    }
}

impl Command for Details {
    // todo: print test logs as well
    fn execute<W: Workspace, A: BacaApi>(self) -> Result<()> {
        info!("Printing details for submit: {}", self.submit_id);

        let instance = W::read_instance()?;
        let submit = A::get_submit_details(&instance, &self.submit_id)?;
        let submit = Submit::parse(&instance, &submit);

        submit.print();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_service::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    fn make_mock_instance() -> InstanceData {
        InstanceData {
            host: "host".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: "perm".to_string(),
            cookie: "invalid".to_string(),
        }
    }

    #[test]
    #[serial]
    fn success_test() {
        let ctx_read = MockWorkspace::read_instance_context();
        ctx_read.expect().returning(|| Ok(make_mock_instance()));

        let ctx_api = MockBacaApi::get_submit_details_context();
        ctx_api
            .expect()
            .once()
            .withf(|x, id| *x == make_mock_instance() && id == "2888")
            .returning(|_, _| Ok(r#"//OK[0,10,65,2,5,7,64,2,5,7,63,2,5,7,62,2,5,7,61,2,5,7,60,2,5,7,59,2,5,7,58,2,5,7,57,2,5,37,56,2,5,37,55,2,5,7,54,2,5,7,53,2,5,7,52,2,5,7,51,2,5,7,50,2,5,7,49,2,5,7,48,2,5,7,47,2,5,7,46,2,5,19,4,3,0,45,44,43,42,41,40,39,38,8,5,37,36,35,34,33,32,31,30,8,5,1,4,3,29,0,28,27,26,25,24,23,22,21,20,9,5,19,18,17,16,15,14,13,12,11,9,5,1,4,3,0,0,10,9,2,5,7,8,2,5,7,6,2,5,2,4,3,2,1,["testerka.gwt.client.submits.SubmitDetailsModel/2564112456","\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\nLogi kolejnej kompilacji:\n\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\nmake: Entering directory `/var/lib/baca/work\x27\ng++ main.cpp source.cpp -std\x3Dc++11 -O2 -o out -static -m32\nmake: Leaving directory `/var/lib/baca/work\x27\n\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\nLogi kolejnej kompilacji:\n\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\x3D\nmake: Entering directory `/var/lib/baca/work\x27\ng++ main.cpp source.cpp -std\x3Dc++11 -O2 -o out -static -m32\nmake: Leaving directory `/var/lib/baca/work\x27\n","testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","748","program zaakceptowany","720","czas","status","[D] Skalowany Gauss","4","2020-04-15 15:30:32","2020-04-30 23:00:00","2020-05-15 23:00:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","Limit pamieci na kompilacje (MB)","Limit kodu zrodlowego (kB)","// Hubert Jaremko\r\n#define NDEBUG\r\n#include \"vectalg.h\"\r\n#include \x3Calgorithm\x3E\r\n#include \x3Cnumeric\x3E\r\n\r\nVector operator+( const Vector\x26 lhs, const Vector\x26 rhs )\r\n{\r\n    auto result \x3D Vector( lhs.size() );\r\n    std::transform( lhs.begin(),\r\n                    lhs.end(),\r\n                    rhs.begin(),\r\n                    result.begin(),\r\n                    std::plus\x3Cdouble\x3E() );\r\n    return result;\r\n}\r\n\r\nclass equation_solver\r\n{\r\npublic:\r\n    equation_solver( const Matrix\x26 a_0, const Vector\x26 b_0, double eps_ )\r\n        : A0( a_0 ), b0( b_0 ), eps( eps_ ), perm_vec( A0.size() ),\r\n          row_norms( A0.size() )\r\n    {\r\n        std::iota( perm_vec.begin(), perm_vec.end(), 0 );\r\n    }\r\n\r\n    Vector solve()\r\n    {\r\n        calculate_lu();\r\n        auto sol \x3D Vector(A.size());\r\n        solve_system_upper( bx, sol );\r\n        return permute( fix_solution( sol ) );\r\n//        return permute( fix_solution( solve_system_upper( bx ) ) );\r\n        //        return fix_solution( solve_system_upper( bx ) );\r\n    }\r\n\r\nprivate:\r\n    Vector permute( const Vector\x26 v )\r\n    {\r\n        auto result \x3D Vector( v.size() );\r\n\r\n        for ( int i \x3D 0; i \x3C v.size(); ++i )\r\n        {\r\n            result[ i ] \x3D v[ perm_vec[ i ] ];\r\n        }\r\n\r\n        return result;\r\n    }\r\n\r\n    double\x26 at( size_t i, size_t j )\r\n    {\r\n        return A( perm_vec[ i ], j );\r\n    }\r\n\r\n    double at( size_t i, size_t j ) const\r\n    {\r\n        return A( perm_vec[ i ], j );\r\n    }\r\n\r\n    double\x26 arr_at( Vector\x26 cont, size_t i )\r\n    {\r\n        return cont[ perm_vec[ i ] ];\r\n    }\r\n\r\n    double arr_at( const Vector\x26 cont, size_t i ) const\r\n    {\r\n        return cont[ perm_vec[ i ] ];\r\n    }\r\n\r\n    void calculate_row_norms()\r\n    {\r\n        for ( int i \x3D 0; i \x3C A.size(); ++i )\r\n        {\r\n            auto norm \x3D 0.0;\r\n            for ( int j \x3D 0; j \x3C A.size(); ++j )\r\n            {\r\n                norm \x3D std::max( norm, std::abs( at( i, j ) ) );\r\n            }\r\n            arr_at( row_norms, i ) \x3D norm;\r\n        }\r\n    }\r\n\r\n    size_t pick_main_element( size_t k ) const\r\n    {\r\n        auto max_elem \x3D k;\r\n\r\n        for ( auto i \x3D k; i \x3C A.size(); ++i )\r\n        {\r\n            const auto scale \x3D std::abs( at( i, k ) ) / arr_at( row_norms, i );\r\n            if ( scale \x3E\r\n                 std::abs( at( max_elem, k ) ) / arr_at( row_norms, max_elem ) )\r\n            {\r\n                max_elem \x3D i;\r\n            }\r\n        }\r\n\r\n        return max_elem;\r\n    }\r\n\r\n    void eliminate( size_t k )\r\n    {\r\n        for ( auto i \x3D k + 1; i \x3C A.size(); ++i )\r\n        {\r\n            const auto multiplier \x3D at( i, k ) / at( k, k );\r\n            at( i, k ) \x3D multiplier;\r\n\r\n            for ( auto j \x3D k + 1; j \x3C A.size(); ++j )\r\n            {\r\n                at( i, j ) -\x3D multiplier * at( k, j );\r\n            }\r\n\r\n            arr_at( bx, i ) -\x3D multiplier * arr_at( bx, k );\r\n        }\r\n    }\r\n\r\n    void calculate_lu()\r\n    {\r\n        calculate_row_norms();\r\n\r\n        for ( auto k \x3D 0; k \x3C A.size() - 1; ++k )\r\n        {\r\n            auto p \x3D pick_main_element( k );\r\n            std::swap( perm_vec[ p ], perm_vec[ k ] );\r\n            eliminate( k );\r\n        }\r\n    }\r\n\r\n//    Vector solve_system_upper( const Vector\x26 b )\r\n    void solve_system_upper( const Vector\x26 b, Vector\x26 result )\r\n    {\r\n        const auto n \x3D A.size();\r\n//        Vector result( n );\r\n\r\n        for ( int i \x3D n - 1; i \x3E\x3D 0; --i )\r\n        {\r\n            auto sum \x3D 0.0;\r\n            for ( auto j \x3D i + 1; j \x3C n; ++j )\r\n            {\r\n                sum +\x3D at( i, j ) * arr_at( result, j );\r\n            }\r\n\r\n            arr_at( result, i ) \x3D ( arr_at( b, i ) - sum ) / at( i, i );\r\n        }\r\n\r\n//        return result;\r\n    }\r\n\r\n//    Vector solve_system_lower( const Vector\x26 b )\r\n    void solve_system_lower( const Vector\x26 b, Vector\x26 result )\r\n    {\r\n        const auto n \x3D A.size();\r\n//        Vector result( n );\r\n\r\n        for ( int i \x3D 0; i \x3C n; ++i )\r\n        {\r\n            auto sum \x3D 0.0;\r\n            for ( auto j \x3D 0; j \x3C i; ++j )\r\n            {\r\n                sum +\x3D at( i, j ) * arr_at( result, j );\r\n            }\r\n\r\n            arr_at( result, i ) \x3D arr_at( b, i ) - sum;\r\n        }\r\n\r\n//        return result;\r\n    }\r\n\r\n    //    Vector residual_vector( const Vector\x26 x ) const\r\n    void residual_vector( const Vector\x26 x, Vector\x26 result ) const\r\n    {\r\n        const auto n \x3D b0.size();\r\n        //        Vector result( n );\r\n        for ( int i \x3D 0; i \x3C n; ++i )\r\n        {\r\n            long double s \x3D 0.0;\r\n            for ( int j \x3D 0; j \x3C n; ++j )\r\n            {\r\n                s +\x3D static_cast\x3Clong double\x3E( A0( i, j ) ) * arr_at( x, j );\r\n            }\r\n            long double bb \x3D b0[ i ];\r\n            result[ i ] \x3D static_cast\x3Cdouble\x3E( bb - s );\r\n        }\r\n//        return result;\r\n    }\r\n\r\n    Vector fix_solution( Vector x0 )\r\n    {\r\n        //        auto residual \x3D residual_vector( x0 );\r\n        auto residual \x3D Vector( A.size() );\r\n        auto result \x3D Vector( A.size() );\r\n        residual_vector( x0, residual );\r\n\r\n        while ( residual.max_norm() \x3E\x3D eps )\r\n        {\r\n            solve_system_lower( residual,result );\r\n            solve_system_upper( result,result );\r\n            //            x0 \x3D x0 + solve_system_upper( solve_system_lower( residual ) );\r\n            //            residual \x3D residual_vector( x0 );\r\n            x0 \x3D x0 + result;\r\n            residual_vector( x0, residual );\r\n        }\r\n\r\n        return x0;\r\n    }\r\n\r\n    const Matrix\x26 A0;\r\n    const Vector\x26 b0;\r\n    Vector bx { b0 };\r\n    Matrix A { A0 };\r\n    double eps { 0.0 };\r\n    Vector perm_vec;\r\n    Vector row_norms;\r\n};\r\n\r\nVector solveEquations( const Matrix\x26 A0, const Vector\x26 b0, double eps )\r\n{\r\n    return equation_solver( A0, b0, eps ).solve();\r\n}\r\n","2888","C++","2020-04-22 13:04:36","5752","89","100","3.58","przekroczony czas","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","testy_jawne/test1","testy_jawne/test2","testy_jawne/test3","testy_jawne/test4","testy_jawne/test5","testy_jawne/test6","testy_jawne/test8","testy/test0","testy/test1","testy/test10","testy/test11","testy/test2","testy/test3","testy/test4","testy/test5","testy/test6","testy/test7","testy/test8","testy/test9","test"],0,7]"#.to_string()));

        let details = Details {
            submit_id: "2888".to_string(),
        };
        let result = details.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }
}
